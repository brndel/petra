use leptos::*;

use crate::component::{loading::Loading, icon::Icons};



#[derive(Clone, PartialEq)]
pub enum ButtonStatus {
    Default,
    Loading,
    Done,
    Error,
}

impl ButtonStatus {
    pub fn get_class(&self) -> &'static str {
        match self {
            ButtonStatus::Default => "primary",
            ButtonStatus::Loading => "primary",
            ButtonStatus::Done => "positive",
            ButtonStatus::Error => "error",
        }
    }

    pub fn get_view<V: IntoView>(&self, default: V) -> View {
        match self {
            ButtonStatus::Default => default.into_view(),
            ButtonStatus::Loading => view! {<Loading/>}.into_view(),
            ButtonStatus::Done => Icons::Valid.into_view(),
            ButtonStatus::Error => Icons::Error.into_view(),
        }
    }
}