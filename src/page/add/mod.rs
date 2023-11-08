mod edit_payment;
mod tink_button;

use chrono::{DateTime, Local};
use leptos::{logging::log, *};

use crate::{
    api::{
        category::Category,
        payment::add_payments,
        rule::Rule,
        user::User,
    },
    component::{
        amount::Amount,
        icon::{Icon, Icons},
        loading::Loading,
        select_menu::MultiSelectMenu,
        user::UserView, util::button_status::ButtonStatus,
    },
    provider::{Me, Provider}, util::calculated_amount::CalculatedAmount,
};

use self::{edit_payment::EditPayment, tink_button::TinkButton};

// #[derive(Clone)]
// enum UploadStatus {
//     Default,
//     Uploading,
//     Done,
//     Error,
// }

// impl UploadStatus {
//     fn get_class(&self) -> &'static str {
//         match self {
//             UploadStatus::Default => "primary",
//             UploadStatus::Uploading => "primary",
//             UploadStatus::Done => "positive",
//             UploadStatus::Error => "error",
//         }
//     }
// }

// impl IntoView for UploadStatus {
//     fn into_view(self) -> View {
//         match self {
//             UploadStatus::Default => "Hochladen".into_view(),
//             UploadStatus::Uploading => view! {<Loading/>}.into_view(),
//             UploadStatus::Done => view! {<Icon icon=Icons::Valid/>}.into_view(),
//             UploadStatus::Error => view! {<Icon icon=Icons::Error/>}.into_view(),
//         }
//     }
// }

#[component]
pub fn AddPage() -> impl IntoView {
    Provider::<Category>::provide();
    Provider::<User>::provide();
    Provider::<Rule>::provide();

    let payments = RwSignal::new(Vec::<EditPayment>::new());
    let upload_status = RwSignal::new(ButtonStatus::Default);

    let payment_count = move || payments.with(|p| p.len());

    let add_payment = move || payments.update(|payments| payments.push(EditPayment::new()));
    let upload_callback = move |result: Result<(), ServerFnError>| match result {
        Ok(_) => {
            upload_status.set(ButtonStatus::Done);
            payments.set(Vec::new());
        }
        Err(err) => {
            upload_status.set(ButtonStatus::Error);
            log!("{:?}", err);
        }
    };

    view! {
        <div class="side left col load-anim">
            <TinkButton payments/>

            <div class="spacer"/>

            <div class="button-bar">
                <button class="primary" on:click=move |_| add_payment() >"Neu"</button>
            </div>
        </div>

        <main class="col">
        <Suspense fallback=||view!{<Loading/>}>
            <For each={move || payments.get()} key=|signal| signal.clone() let:payment>
                <AddPaymentView payment/>
            </For>
        </Suspense>
        </main>

        <div class="side right col load-anim">
            <div class="card col center">
                {move || {
                    let me_prov = Provider::<Me>::expect();

                    let amount = me_prov.get_single().map(
                        |me| payments.with(
                            |payments| payments.iter().map(|payment| payment.get_amount(&me.id)).fold(CalculatedAmount::default(), |a, b| a + b)
                        )
                    ).unwrap_or_default();

                    // log!("payments: {}, errors: {}", payment_count, error_count);

                    view! {
                        <span class="row"><b>{payment_count()}</b> "Zahlungen"</span>
                        <div class="row center">
                            <Amount amount=amount.user_amount/>
                            <Icon icon=Icons::Repay/>
                            <Amount amount=amount.repay_amount/>
                        </div>
                    }
                }}
            </div>

            <div class="spacer"/>

            <div class="card col center">
            {move || {
                let payment_count = payment_count();
                let error_count: u32 = payments.with(|payments|
                    payments.iter().map(
                        |payment| if payment.is_valid() {0} else {1}
                    ).sum()
                );

                view! {
                    {if error_count > 0 { Some(view! {
                        <span class="card row center error">
                            <b>{error_count}</b>
                            "Fehler"
                        </span>
                    })} else {None}}
                    <div class="button-bar">
                        <button class={move || upload_status.get().get_class()} disabled={error_count > 0 && payment_count > 0} on:click=move |_| {
                            let res = upload_payments(payments, upload_callback);

                            if res.is_ok() {
                                upload_status.set(ButtonStatus::Loading);
                            } else {
                                upload_status.set(ButtonStatus::Error);
                            }

                        } >{move || upload_status.get().get_view("Hochladen")}</button>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}

fn upload_payments<
    S: SignalGetUntracked<Value = Vec<EditPayment>>,
    F: Fn(Result<(), ServerFnError>) + 'static,
>(
    payments: S,
    upload_callback: F,
) -> Result<(), ()> {
    let payments = Option::<Vec<_>>::from_iter(
        payments
            .get_untracked()
            .iter()
            .map(|payment| payment.try_into().ok()),
    )
    .ok_or(())?;

    spawn_local(async move {
        let res = add_payments(payments).await;

        upload_callback(res);
    });

    Ok(())
}

#[component]
fn AddPaymentView(payment: EditPayment) -> impl IntoView {
    let category_prov = Provider::<Category>::expect();
    let user_prov = Provider::<User>::expect();
    let me_prov = Provider::<Me>::expect();
    let rule_prov = Provider::<Rule>::expect();

    let rule = payment
        .import_data
        .as_ref()
        .and_then(|data| data.rule.clone());

    let name_valid = move || payment.name.with(|name| name.len() > 0);
    let users_valid = move || payment.users.with(|users| users.len() > 0);

    let payment_valid = move || name_valid() && users_valid();

    view! {
        <div class="card col">
            <div class="row">
                <div class="row spacer2 center">
                    {move || {
                        if payment_valid() {
                            view! {
                                <div class="circle positive">
                                    <Icon icon=Icons::Valid/>
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div class="circle error">
                                    <Icon icon=Icons::Error/>
                                </div>
                            }.into_view()
                        }
                    }}
                    <input class="spacer" type="text" placeholder="Name" minlength="1"
                        class:err=move || !name_valid()
                        prop:value=move || payment.name.get_untracked()
                        on:input=move |ev| payment.name.update(move |name| *name = event_target_value(&ev))
                    />
                    {move || rule.as_ref().and_then(move |id| rule_prov.get(id)).map(move |rule| {
                        view! {
                            <Icon icon=Icons::Rule tooltip={rule.name}/>
                        }
                    })}
                </div>

                <div class="row spacer center">
                    {move || view!{
                        <MultiSelectMenu
                            name=move || view!{"Categories"}
                            signal=payment.categories
                            items={category_prov.get_all().unwrap_or_default()}
                            // sort_key=move |v| category_prov.get(v).map(|c| c.name).unwrap_or_default()
                        />
                    }}

                    {move || {
                        payment.categories.get().into_iter().map(|c| view!{<Icon icon={category_prov.get(&c).map(|c| c.icon).unwrap_or_default()}/>}).collect_view()
                    }}
                </div>


                <div class="row spacer end">
                    <input type="number" step="0.01"
                    class:err=move || payment.amount.with(|amount| amount.is_none())
                    prop:value=move || payment.amount.with_untracked(|amount| format!("{:.2}",(amount.unwrap_or_default() as f32 / 100.0)))
                    on:change=move |ev| {
                        let target = event_target_value(&ev);

                        let amount = match target.parse::<f32>() {
                            Ok(value) =>  Some((value * 100.0) as i64),
                            Err(_) => None,
                        };
                        payment.amount.update(|value| *value = amount);
                    } />
                </div>
            </div>
            {payment.import_data.as_ref().map(move |import_data| view!{
                <div class="col light">
                    <div class="row center">
                        <span>{import_data.tink.name.clone()}</span>
                        <span>{import_data.tink.raw_name.clone()}</span>
                    </div>
                    {import_data.tink.counterparties.as_ref().map(|cp| view! {
                        <div class="row center">
                            <span>{cp.payer.name.clone()}</span>
                            <Icon icon=Icons::ArrowRight/>
                            <span>{cp.payee.name.clone()}</span>
                        </div>
                    })}
                </div>
            })}
            <div class="row">
                <div class="row spacer2">
                    <input class="spacer" type="datetime-local"
                        class:err=move || payment.date.with(|amount| amount.is_none())
                        prop:value=move || {
                            let mut date = payment.date.with_untracked(|date| date.unwrap_or_else(|| Local::now().fixed_offset()).to_rfc3339());
                            let _ = date.split_off(16);
                            date
                        }
                        on:input=move |ev| {
                            let mut target = event_target_value(&ev);

                            target += ":00-00:00";

                            let date = DateTime::parse_from_rfc3339(&target).ok();

                            log!("date: {:?}", date);

                            payment.date.update(|value| *value = date);
                    } />
                </div>

                <div
                    class="row spacer center"
                    class:child-err=move || !users_valid()
                >
                    {move || view!{
                        <MultiSelectMenu
                            name=||"Users"
                            signal=payment.users
                            items={user_prov.get_all().unwrap_or_default()}
                            // sort_key=move |v| user_prov.get(v).map(|u| u.name).unwrap_or_default()
                        />
                    }}

                    {move || {
                        payment.users.get().into_iter().map(|u| user_prov.get(&u).map(|user| view!{<UserView user=&user/>})).collect_view()
                    }}
                </div>

                <div class="row spacer end center">
                    {move || {
                        let amount = me_prov.get_single().map(|me| payment.get_amount(&me.id)).unwrap_or_default();

                        view! {
                            <Amount amount=amount.user_amount/>
                            <Icon icon=Icons::Repay/>
                            <Amount amount=amount.repay_amount/>
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
