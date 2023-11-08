use leptos::*;

use crate::{component::icon::Icon, api::category::Category};

#[component]
pub fn CategoryView<'a>(category: &'a Category) -> impl IntoView {
    view! {
        <Icon icon={category.icon.clone()} tooltip={category.name.clone()} />
    }
}
