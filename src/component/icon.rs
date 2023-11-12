use leptos::*;

#[component]
pub fn Icon<T: AsRef<str>>(
    icon: T,
    #[prop(optional)] tooltip: String,
    #[prop(optional)] size: IconSize,
) -> impl IntoView {
    view! {
        <i class={format!("fa-solid fa-fw fa-{} {}", icon.as_ref(), size.as_ref())} data-tooltip=tooltip/>
    }
}

#[allow(dead_code)]
pub enum IconSize {
    Small,
    Default,
    Big,
}

impl Default for IconSize {
    fn default() -> Self {
        Self::Default
    }
}

impl AsRef<str> for IconSize {
    fn as_ref(&self) -> &str {
        match self {
            IconSize::Small => "fa-2xs",
            IconSize::Default => "",
            IconSize::Big => "fa-2xl",
        }
    }
}

#[allow(dead_code)]
pub enum Icons {
    ArrowLeft,
    ArrowRight,
    ArrowDown,
    ArrowUp,
    Repay,
    Info,
    Search,
    OpenLink,
    ExpandArrow,
    Rule,
    Category,
    Payment,
    AddPayment,
    Home,
    Valid,
    Error,
    Imported,
    Add,
    Delete,
}

impl AsRef<str> for Icons {
    fn as_ref(&self) -> &str {
        match self {
            Icons::ArrowLeft => "arrow-left",
            Icons::ArrowRight => "arrow-right",
            Icons::ArrowDown => "arrow-down",
            Icons::ArrowUp => "arrow-up",
            Icons::Repay => "arrows-rotate",
            Icons::Info => "info",
            Icons::Search => "magnifying-glass",
            Icons::OpenLink => "arrow-up-right-from-square",
            Icons::ExpandArrow => "angle-down",
            Icons::Rule => "robot",
            Icons::Category => "shapes",
            Icons::Payment => "money-bill-wave",
            Icons::AddPayment => "cash-register",
            Icons::Home => "house",
            Icons::Valid => "check",
            Icons::Error => "xmark",
            Icons::Imported => "cloud-arrow-down",
            Icons::Add => "add",
            Icons::Delete => "trash-can",
        }
    }
}

impl IntoView for Icons {
    fn into_view(self) -> View {
        view! {
            <Icon icon=self/>
        }
    }
}