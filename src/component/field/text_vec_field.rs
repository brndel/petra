use leptos::*;

use crate::component::icon::Icons;

#[component]
pub fn TextVecField<
    S: SignalWith<Value = Vec<String>> + SignalUpdate<Value = Vec<String>> + Copy + 'static,
>(
    signal: S,
) -> impl IntoView {
    view! {
        {move || signal.with(
                move |texts| texts.iter().enumerate().map(move |(i, text)| view! {
                    <div class="row">
                        <input
                            class="spacer"
                            type="text"
                            prop:value=text
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                signal.update(|vec| vec[i] = value);
                            }
                        />
                        <button on:click=move |_| signal.update(|vec| {vec.remove(i);})>
                            {Icons::Delete}
                        </button>
                    </div>
                }).collect_view()
            )
        }
        <button on:click=move |_| signal.update(|vec| vec.push(String::new()))>
            {Icons::Add}
        </button>
    }
}
