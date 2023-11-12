use leptos::*;
use core::fmt::Debug;

#[component]
pub fn Error<E: SignalGet<Value = D> + 'static, D: Debug>(error: E) -> impl IntoView {
    view! {
        <div class="error-view">
            {move || format!("{:?}", error.get())}
        </div>
    }
}
