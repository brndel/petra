use leptos::*;

#[component]
pub fn TextField<S: SignalSet<Value = String> + SignalGet<Value = String> + Copy + 'static>(
    signal: S,
    #[prop(optional)]
    style: TextFieldStyle
) -> impl IntoView {
    view! {
        <input
            type="text"
            class=style.class()
            placeholder=style.placeholder()
            prop:value=move || signal.get()
            on:input=move |ev| signal.set(event_target_value(&ev))
        />
    }
}


pub enum TextFieldStyle {
    Default,
    Search
}

impl Default for TextFieldStyle {
    fn default() -> Self {
        Self::Default
    }
}

impl TextFieldStyle {
    fn class(&self) -> &'static str {
        match self {
            TextFieldStyle::Default => "",
            TextFieldStyle::Search => "margin-small",
        }
    }

    fn placeholder(&self) -> &'static str {
        match self {
            TextFieldStyle::Default => "",
            TextFieldStyle::Search => "Suche...",
        }
    }
}