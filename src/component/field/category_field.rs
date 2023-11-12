use leptos::*;
use mensula_key::Key;

use crate::{
    api::category::Category,
    component::{category::CategoryView, select_menu::MultiSelectMenu},
    provider::Provider,
};

#[component]
pub fn CategoryField<
    S: SignalUpdate<Value = Vec<Key>> + SignalWith<Value = Vec<Key>> + Copy + 'static,
>(
    signal: S,
) -> impl IntoView {
    let category_prov = Provider::<Category>::expect();

    view! {
        {move || view! {
            <MultiSelectMenu
                name=move || "Categories"
                signal
                items={category_prov.get_all().unwrap_or_default()}
            />
        }}
        <div class="row center center-j">
            {move || {
                signal.with(
                    move |categories| categories.iter().map(
                        move |c| category_prov.get(c).map(
                            |category|
                                view! {
                                    <CategoryView category=&category/>
                                }
                        )
                    ).collect_view()
                )
            }}
        </div>
    }
}
