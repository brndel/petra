use leptos::*;
use mensula_key::Key;

use crate::{
    api::category::CategoryGroup,
    component::select_menu::SelectMenu,
    provider::Provider,
};

#[component]
pub fn CategoryGroupField<S: SignalSet<Value = Key> + SignalGet<Value = Key> + Copy + 'static>(
    signal: S,
) -> impl IntoView {
    let group_prov = Provider::<CategoryGroup>::expect();

    move || {
        view! {
            <SelectMenu
                signal
                items={group_prov.get_all().unwrap_or_default()}
            />
        }
    }
}
