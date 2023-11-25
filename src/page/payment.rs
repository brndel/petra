use leptos::*;
use leptos_router::{use_params_map, use_query_map, Outlet, A};
use mensula_key::Key;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        payment::{
            get_months, get_payment, get_payments, Payment, PaymentMonthData,
        },
        user::User, category::Category,
    },
    component::{
        amount::Amount,
        category::CategoryView,
        icon::Icons,
        payment::PaymentView,
        response_builder::ResponseBuilder,
        user::UserView,
    },
    util::{lang::Translate, month::MonthDate, calculated_amount::CalculatedAmount}, provider::Provider,
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
fn PaymentDetails() -> impl IntoView {
    let query = use_query_map();

    let payment_id = move || query.with(|query| query.get("payment").cloned());

    let payment = create_resource(payment_id, move |id| fetch_payment(id));

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
    let owner = payment.owner.clone();
    let owner = move || user_prov.get(&owner);
    let amounts = payment.get_all_amounts().clone();
    let amounts = move || map_amounts(&user_prov, &amounts);

    let category_prov = Provider::<Category>::expect();
    let categories = payment.categories.clone();
    let categories = move || category_prov.get_multiple(&categories);

    view! {
        {move || match (owner(), amounts(), categories()) {
            (Some(owner), Some(amounts), Some(categories)) => Some(view! {
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
            }),
            _ => None,
        }}
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