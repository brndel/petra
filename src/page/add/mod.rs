mod edit_payment;
mod edit_payment_view;
mod tink_button;

use leptos::{logging::log, *};

use crate::{
    api::{category::Category, payment::add_payments, rule::Rule, user::User},
    component::{
        amount::Amount,
        icon::Icons,
        loading::Loading,
        util::button_status::ButtonStatus,
    },
    page::add::edit_payment_view::EditPaymentView,
    provider::{Me, Provider},
    util::calculated_amount::CalculatedAmount,
};

use self::{edit_payment::EditPayment, tink_button::TinkButton};

#[component]
pub fn AddPage() -> impl IntoView {
    Provider::<Category>::provide();
    Provider::<User>::provide();
    Provider::<Rule>::provide();

    let payments = RwSignal::new(Vec::<EditPayment>::new());
    let upload_status = RwSignal::new(ButtonStatus::Default);

    let payment_count =
        move || payments.with(|p| p.iter().filter(|payment| payment.enabled.get()).count());
    let disabled_count = 
        move || payments.with(|p| p.iter().filter(|payment| !payment.enabled.get()).count());

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
                <EditPaymentView payment/>
            </For>
        </Suspense>
        </main>

        <div class="side right col load-anim">
            <div class="card col center">
                {move || {
                    let me_prov = Provider::<Me>::expect();

                    let amount = me_prov.get_single().map(
                        |me| payments.with(
                            |payments| payments.iter().filter_map(|payment|
                                if payment.enabled.get() {
                                    Some(payment.get_amount(&me.id))
                                } else {
                                    None
                                }
                            ).fold(CalculatedAmount::default(), |a, b| a + b)
                        )
                    ).unwrap_or_default();

                    // log!("payments: {}, errors: {}", payment_count, error_count);

                    view! {
                        <span class="row"><b>{payment_count()}</b> "Zahlungen"</span>
                        <div class="row center">
                            <Amount amount=amount.user_amount/>
                            {Icons::Repay}
                            <Amount amount=amount.repay_amount/>
                        </div>
                        {move || {
                            let disabled = disabled_count();

                            if disabled != 0 {
                                Some(view!{
                                    <span class="row light"><b>{disabled}</b> "deaktivierte Zahlungen"</span>
                                })
                            } else {
                                None
                            }
                        }}
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
                        <button
                            class={move || upload_status.get().get_class()}
                            disabled={error_count > 0 || payment_count == 0}
                        on:click=move |_| {
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
