use leptos::*;

use super::field::{FieldReset, FieldUnchaged};

#[component]
pub fn SubmitField<F: FieldReset + FieldUnchaged + Copy + 'static, S: Fn() + 'static>(
    field: F,
    on_submit: S,
) -> impl IntoView {
    view! {
        <div class="button-bar">
            <button
                on:click=move |_| field.reset()
                disabled=move || field.is_unchanged()
            >"Cancel"</button>
            <button
                class="primary"
                on:click=move |_| on_submit()
                disabled=move || field.is_unchanged()
            >"Update"</button>
        </div>
    }
}
