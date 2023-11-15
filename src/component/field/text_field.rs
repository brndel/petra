use leptos::*;

#[component]
pub fn TextField<S: SignalSet<Value = String> + SignalGet<Value = String> + Copy + 'static>(
    signal: S,
    #[prop(optional)]
    style: TextFieldStyle
) -> impl IntoView {
    view! {
        <input
            type=style.input_type()
            class=style.class()
            placeholder=style.placeholder()
            prop:value=move || signal.get()
            on:input=move |ev| signal.set(event_target_value(&ev))
        />
    }
}


pub enum TextFieldStyle {
    Default,
    Search,
    Password,
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
            TextFieldStyle::Password => "",
        }
    }

    fn placeholder(&self) -> &'static str {
        match self {
            TextFieldStyle::Default => "",
            TextFieldStyle::Search => "Suche...",
            TextFieldStyle::Password => "",
        }
    }

    fn input_type(&self) -> &'static str {
        match self {
            TextFieldStyle::Password => "password",
            _ => "text",
        }
    }
}