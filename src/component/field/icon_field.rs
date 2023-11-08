use leptos::*;

use crate::component::icon::{Icon, IconSize, Icons};

#[component]
pub fn IconField<S: SignalSet<Value = String> + SignalGet<Value = String> + Copy + 'static>(signal: S) -> impl IntoView {

    view! {
        <div class="row center">
            <input
                type="text"
                class="spacer"
                prop:value=move || signal.get()
                on:input=move |ev| signal.set(event_target_value(&ev))
            />
            <a href="https://fontawesome.com/search?o=r&m=free&s=solid" target="_blank" rel="noreferrer noopener">
                <Icon icon=Icons::OpenLink/>
            </a>
        </div>
        <div class="col center">
            {move || view!{<Icon icon={signal.get()} size=IconSize::Big/>}}
        </div>
    }
}