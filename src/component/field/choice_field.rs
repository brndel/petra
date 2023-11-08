use std::marker::PhantomData;

use leptos::*;

use crate::component::select_menu::{SelectMenu, MenuItem};

pub trait Choose: Clone {
    fn options() -> &'static [Self];
}

#[component]
pub fn ChoiceField<
    T: Choose + Into<MenuItem<T>> + Clone + PartialEq + 'static,
    S: SignalGet<Value = T> + SignalSet<Value = T> + Copy + 'static,
>(
    signal: S,
    #[prop(optional)]
    _phantom: PhantomData<T>,
) -> impl IntoView {
    view! {
        <SelectMenu
            signal
            items={T::options().to_owned()} />
    }
}
