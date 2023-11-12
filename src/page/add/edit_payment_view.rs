use chrono::{DateTime, Local};
use leptos::{*, logging::log};

use crate::{
    api::{category::Category, rule::Rule, user::User},
    page::add::edit_payment::EditPayment,
    provider::{Me, Provider}, component::{icon::{Icon, Icons}, select_menu::MultiSelectMenu, amount::Amount, user::UserView},
};

#[component]
pub fn EditPaymentView(payment: EditPayment) -> impl IntoView {
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
        <div class="card col" class:light=move || !payment.enabled.get()>
            <div class="row">
                <div class="row spacer2 center">
                    {move || {
                        let is_valid = payment_valid();

                        view! {
                            <button
                                class={format!("circle {}", if is_valid {"positive"} else {"error"})}
                                on:click=move |_| {payment.enabled.update(|v| *v = !*v)}
                            >
                                <Icon icon={if is_valid {Icons::Valid} else {Icons::Error}}/>
                            </button>
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
                            {Icons::ArrowRight}
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
                            {Icons::Repay}
                            <Amount amount=amount.repay_amount/>
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
