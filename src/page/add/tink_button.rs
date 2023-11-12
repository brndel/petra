use std::time::Duration;

use chrono::{Datelike, Local};
use leptos::{logging::log, *};
use mensula_key::Key;

use crate::{
    api::{
        rule::Rule,
        tink::{tink_get_payments, tink_get_token_timeout},
        user::User,
    },
    component::{
        field::choice_field::ChoiceField, response_builder::ResponseBuilder,
        util::button_status::ButtonStatus,
    },
    provider::{Me, Provider},
    util::{
        lang::Translate,
        month::{Month, MonthDate},
    },
};

use super::EditPayment;

const TINK_URL: &str = "https://link.tink.com/1.0/transactions/connect-accounts/?client_id=54e8e5d65f5e4339ad76321d45c0f990&redirect_uri=http%3A%2F%2Flocalhost%3A8187%2Fapi%2Ftink%2Fcallback&market=DE&locale=de_DE";

#[component]
pub fn TinkButton<S: SignalSet<Value = Vec<EditPayment>> + Copy + 'static>(
    payments: S,
) -> impl IntoView {
    let tink_timeout = create_local_resource(|| (), |_| tink_get_token_timeout());
    let button_status = RwSignal::new(ButtonStatus::Default);

    let rule_prov = Provider::<Rule>::expect();
    let user_prov = Provider::<User>::expect();
    let me_prov = Provider::<Me>::expect();

    let on_start = move || {button_status.set(ButtonStatus::Loading)};

    let on_error = move || {button_status.set(ButtonStatus::Error)};

    let on_response = move |new_payments: Vec<EditPayment>| {
        button_status.set(ButtonStatus::Done);
        payments.set(new_payments);
    };

    view! {
        <div class="card col stretch">
            <h3 class="center">"Tink"</h3>

            <ResponseBuilder res=tink_timeout builder=move |timeout| {
                match timeout {
                    Some(timeout) => {
                        let now = Local::now().fixed_offset();

                        let year = create_rw_signal(now.year());
                        let month = create_rw_signal(Month::try_from(now.month() as u8).unwrap());
                        let month_date = move || MonthDate::new(year.get(), month.get());
                        let month_date_untracked = move || MonthDate::new(year.get_untracked(), month.get_untracked());

                        let now = RwSignal::new(now);
                        let duration = move || timeout - now.get();

                        let handle = set_interval_with_handle(move || now.set(Local::now().fixed_offset()), Duration::from_secs(1)).unwrap();

                        on_cleanup(move || handle.clear());

                        view! {
                            <span class="center">
                                {move || {
                                    let duration = duration();
                                    format!("{:02}:{:02}:{:02}", duration.num_hours(), duration.num_minutes() & 60, duration.num_seconds() % 60)
                                }}
                            </span>


                            <input type="number" step="1" prop:value=move || year.get() on:input={move |ev| match event_target_value(&ev).parse::<i32>() {
                                Ok(v) => year.set(v),
                                Err(_) => (),
                            }}/>

                            <ChoiceField signal=month/>

                            {move || match (rule_prov.get_all(), user_prov.get_all_ids(), me_prov.get_single_id()) {
                                (Some(rules), Some(users), Some(me)) => Some(view! {
                                    <div class="button-bar">
                                        <button
                                            class=move || button_status.get().get_class()
                                            disabled=move || button_status.get() != ButtonStatus::Default
                                            on:click=move |_| {
                                            load_tink_payments(
                                                month_date_untracked(),
                                                on_start,
                                                on_response,
                                                on_error,
                                                rules.clone(),
                                                users.clone(),
                                                me.clone()
                                            );
                                        }>
                                            {move || button_status.get().get_view(move || month_date().translate_default())}
                                        </button>
                                    </div>
                                }),
                                _ => None,
                            }}
                        }.into_view()
                    },
                    None => view! {
                        <div class="button-bar">
                            <a class="button primary" href=TINK_URL>"Verbinden"</a>
                        </div>
                    }.into_view()
                }
            }
            />
        </div>
    }
}

fn load_tink_payments<
    S: Fn() + Copy + 'static,
    R: Fn(Vec<EditPayment>) + Copy + 'static,
    E: Fn() + Copy + 'static,
>(
    month: MonthDate,
    on_start: S,
    on_response: R,
    on_error: E,
    rules: Vec<Rule>,
    users: Vec<Key>,
    me: Key,
) {
    on_start();
    spawn_local(async move {
        let payments = tink_get_payments(month).await;

        let payments = match payments {
            Ok(payments) => payments,
            Err(err) => {
                log!("Error {:?}", err);
                on_error();
                return;
            }
        };

        let new_payments = payments
            .into_iter()
            .map(|p| EditPayment::from_tink_payment(p, &rules, &users, &me))
            .collect();

        on_response(new_payments);
    });
}
