use std::collections::HashMap;

use leptos::*;
use leptos_router::{use_params_map, use_query_map, Outlet, A};
use mensula_key::Key;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        category::{Category, CategoryGroup},
        payment::{get_months, get_payment, get_payments, payment_update_users, Payment, PaymentMonthData},
        user::User,
    },
    component::{
        amount::Amount,
        category::CategoryView,
        icon::{Icon, Icons},
        payment::PaymentView,
        response_builder::ResponseBuilder,
        select_menu::MultiSelectMenu,
        user::UserView,
    },
    provider::{Me, Provider},
    util::{calculated_amount::CalculatedAmount, lang::Translate, month::MonthDate, reload_signal::ReloadSignal},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct MonthButtonData {
    month: MonthDate,
    balance: i64,
}

#[component]
pub fn PaymentPage() -> impl IntoView {
    let months = create_resource(|| (), move |_| get_months());

    Provider::<User>::provide();
    Provider::<Category>::provide();
    Provider::<CategoryGroup>::provide();

    view! {
        <div class="side left col load-anim">
            <ResponseBuilder res={months} builder=|months| months.into_iter().map(|month| view! {<MonthButton data={month}/>}).collect_view()  />
        </div>
        <main class="col load-anim">
            <Outlet/>
        </main>

        <PaymentDetails/>
    }
}

#[component]
fn MonthButton(data: PaymentMonthData) -> impl IntoView {
    view! {
        <A href={data.month.as_string()} class="card col">
            <div class="row center space">
                <span>{data.month.translate_default()}</span>
                <span>{data.payments_count}</span>
            </div>
            <div class="row center">
                <Amount amount={data.amount.calculated_amount()} />
            </div>
        </A>
    }
}

async fn fetch_payments(
    month: Option<MonthDate>,
) -> Result<Option<(Vec<Payment>, MonthDate)>, ServerFnError> {
    match month {
        Some(month) => match get_payments(month.clone()).await {
            Ok(payments) => Ok(Some((payments, month))),
            Err(err) => Err(err),
        },
        None => Ok(None),
    }
}

#[component]
pub fn PaymentPageMain() -> impl IntoView {
    let params = use_params_map();

    let payments = create_resource(
        move || params.get(),
        |params| async move {
            let value = params.get("month").cloned().unwrap_or_default();

            let month = value.parse::<MonthDate>().ok();

            fetch_payments(month).await
        },
    );

    view! {
        <ResponseBuilder res={payments} builder=move |payments| match payments {
            Some((payments, month)) => view! {
                <h2>{month.translate_default()}</h2>
                <MonthStatistics payments=&payments/>
                {
                    payments.into_iter().map(move |payment| view!{<PaymentView payment/>}).collect_view()
                }
            }.into_view(),
            None => ().into_view(),
            }
        />
    }
}

async fn fetch_payment(id: Option<String>) -> Result<Option<Payment>, ServerFnError> {
    let id = match id {
        Some(id) => id,
        None => return Ok(None),
    };

    get_payment(id.into()).await.map(Some)
}

#[component]
fn MonthStatistics<'a>(payments: &'a [Payment]) -> impl IntoView {
    let me_prov = Provider::<Me>::expect();
    let category_prov = Provider::<Category>::expect();
    let category_group_prov = Provider::<CategoryGroup>::expect();

    let category_amount_map = || {
        let me = me_prov.get_single().unwrap();
        payments
            .iter()
            .map(move |payment| (&payment.categories, payment.get_amount(&me.id)))
            .fold(
                HashMap::default(),
                |mut map: HashMap<Key, i64>, (categories, amount)| {
                    if !categories.is_empty() {
                        let amount_per_category =
                            amount.calculated_amount() / (categories.len() as i64);

                        for category in categories {
                            let amount = map.entry(category.clone()).or_insert(0);
                            *amount += amount_per_category;
                        }
                    }

                    map
                },
            )
    };

    view! {
        <div class="card row">
            {
                let category_map = category_amount_map();

                category_group_prov.get_all().unwrap_or_default().iter().map(move |category_group| {
                    let categories = category_group.categories.iter().map(|category| (category, category_map.get(&category).map(ToOwned::to_owned).unwrap_or_default()));

                    let sum_amount = categories.clone().fold(0, |sum, (_, amount)| {sum + amount});

                    view! {
                        <div class="card col spacer">
                            <div class="row space"><Icon icon={&category_group.icon}/> <span class="bold">{&category_group.name}</span> <Amount amount=sum_amount/></div>
                            {
                                categories.map(|(category_id, amount)| {
                                    let category = category_prov.get(category_id);

                                    view! {
                                        <span>{category.map(|category| view! {<CategoryView category=&category/>})} <Amount amount/></span>
                                    }
                                }).collect_view()
                            }
                        </div>

                    }
                }).collect_view()
            }
        </div>
    }
}

#[derive(Clone)]
struct PaymentDetailsReload(ReloadSignal);

#[component]
fn PaymentDetails() -> impl IntoView {
    let query = use_query_map();

    let payment_id = move || query.with(|query| query.get("payment").cloned());

    let reload_signal = ReloadSignal::new();
    provide_context(PaymentDetailsReload(reload_signal));

    let payment = create_resource(move || (payment_id(), reload_signal.get()), move |(id, _)| fetch_payment(id));

    view! {
        <div class="side right col center load-anim">
        <ResponseBuilder res={payment} builder=move |payment| match payment {
            Some(payment) => {
                view! {
                    <PaymentDetailsInner payment/>
                }.into_view()
            },
            None => ().into_view(),
        } />
        </div>
    }
}

#[component]
fn PaymentDetailsInner(payment: Payment) -> impl IntoView {
    let user_prov = Provider::<User>::expect();
    let me_prov = Provider::<Me>::expect();
    let me = move || me_prov.get_single();
    let owner = payment.owner.clone();
    let owner = move || user_prov.get(&owner);
    let amounts = payment.get_all_amounts().clone();
    let amounts = move || map_amounts(&user_prov, &amounts);

    let category_prov = Provider::<Category>::expect();
    let categories = payment.categories.clone();
    let categories = move || category_prov.get_multiple(&categories);

    let show_edit = create_rw_signal(false);
    let users_edit = create_rw_signal(payment.users.clone());

    let (payment_id, _) = create_signal(payment.id.clone());

    let reload_signal = expect_context::<PaymentDetailsReload>().0;

    let payment_owner = payment.owner.clone();
    let is_owner = move || me().map(|user| user.id) == Some(payment_owner.clone());

    move || match (is_owner(), owner(), amounts(), categories()) {
            (is_owner, Some(owner), Some(amounts), Some(categories)) => Some(view! {
                <span class="bold center">
                    {if payment.imported {Some(Icons::Imported)} else {None}}
                    {payment.name.clone()}
                </span>
                <span class="center">
                    {payment.timestamp.clone().translate_default()}
                </span>

                <div class="divider"/>

                <UserAmountCard user=&owner amount=payment.amount />

                {
                    if payment.users.len() == 1 && payment.users[0] == payment.owner {
                        None
                    } else {
                        Some(view! {
                            {Icons::Repay}
                            <div class="row">
                            {
                                amounts.into_iter().map(
                                    |(user, amount)| {
                                        view! {
                                            <UserRepayCard user=&user amount=amount.repay_amount/>
                                        }
                                    }
                                ).collect_view()
                            }
                            </div>
                        })
                    }
                }

                <div class="divider"/>
                <div class="row center">
                {
                    categories.iter().map(|category| view! {
                        <CategoryView category/>
                    }).collect_view()
                }
                </div>

                {move || (is_owner && !show_edit.get()).then(move || view! {
                    <button on:click=move |_| show_edit.set(true)>
                        <Icon icon=Icons::Edit/>
                    </button>
                })}

                {move || show_edit.get().then(move || view! {
                    <div class="row">
                        {move || {
                            users_edit.get().into_iter().map(|u| user_prov.get(&u).map(|user| view!{<UserView user=&user/>})).collect_view()
                        }}
                    </div>

                    <MultiSelectMenu
                        name=|| "Users"
                        signal=users_edit
                        items={user_prov.get_all().unwrap_or_default()}
                    />

                    <button class="primary" on:click=move |_| {
                        spawn_local(async move {
                            match payment_update_users(payment_id.get_untracked().clone(), users_edit.get_untracked()).await {
                                Ok(_) => reload_signal.reload(),
                                _ => println!("error!"),
                            }
                        })
                    }>
                        "Update"
                    </button>
                })}
            }),
            _ => None,
    }
    
}

fn map_amounts(
    user_prov: &Provider<User>,
    amounts: &Vec<(Key, CalculatedAmount)>,
) -> Option<Vec<(User, CalculatedAmount)>> {
    let mut users = Vec::new();

    for (id, amount) in amounts {
        let user = user_prov.get(id)?;
        users.push((user, amount.clone()));
    }

    Some(users)
}

#[component]
fn UserAmountCard<'a>(user: &'a User, amount: i64, #[prop(optional)] repay: bool) -> impl IntoView {
    view! {
        <div class="card row center space self-stretch">
            <UserView user=&user/>
            {if repay { Some(Icons::Repay) } else { None }}
            <Amount amount/>
        </div>
    }
}

#[component]
fn UserRepayCard<'a>(user: &'a User, amount: i64) -> impl IntoView {
    view! {
        <div class="card col center">
            <UserView user=&user/>
            <Amount amount/>
        </div>
    }
}
