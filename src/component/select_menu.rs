use leptos::{
    html::{Div, Input},
    *,
};
use leptos_use::use_element_hover;

use crate::{
    component::icon::{Icon, Icons},
    util::search::search_str,
};

#[component]
pub fn SelectMenu<
    T: Into<MenuItem<V>> + 'static,
    V: Clone + PartialEq + 'static,
    S: SignalSet<Value = V> + SignalGet<Value = V> + Copy + 'static,
>(
    signal: S,
    #[prop(optional)] name: Option<String>,
    items: Vec<T>,
) -> impl IntoView {
    let (items, _) = create_signal(items.into_iter().map(Into::into).collect::<Vec<_>>());

    let callback = move |value: V| signal.set(value);

    let name = name.map(|name| name.into_view()).unwrap_or_else(|| {
        (move || {
            let value = signal.get();
            let items = items.get();

            let item = items.iter().find(|item| item.value == value);

            item.map(|item| {
                view! {
                    {item.icon.clone().map(|icon| view!{<Icon icon/>})}
                    <span>{item.name.clone()}</span>
                }
            })
        })
        .into_view()
    });

    view! {
        <Menu callback name items/>
    }
}

#[component]
pub fn MultiSelectMenu<
    T: Into<MenuItem<V>> + 'static,
    V: Clone + PartialEq + 'static,
    S: SignalUpdate<Value = Vec<V>> + Copy + 'static,
    N: IntoView + 'static,
>(
    signal: S,
    name: N,
    items: Vec<T>,
) -> impl IntoView {
    let (items, _) = create_signal(items.into_iter().map(Into::into).collect::<Vec<_>>());

    let sort_key = move |value: &V| {
        items.with_untracked(|items| {
            items
                .iter()
                .find(|&item| &item.value == value)
                .map(|item| item.name.clone())
                .unwrap_or_default()
        })
    };

    let callback = move |value: V| {
        signal.update(|values| {
            match values.binary_search_by_key(&sort_key(&value), sort_key) {
                Ok(position) => {
                    values.remove(position);
                }
                Err(position) => {
                    values.insert(position, value);
                }
            };
        });
    };

    view! {
        <Menu callback name items/>
    }
}

#[component]
fn Menu<
    V: Clone + 'static,
    F: Fn(V) + Copy + 'static,
    N: IntoView + 'static,
    I: SignalWith<Value = Vec<MenuItem<V>>> + SignalGet<Value = Vec<MenuItem<V>>> + Copy + 'static,
>(
    callback: F,
    name: N,
    items: I,
) -> impl IntoView {
    let hover_ref = create_node_ref::<Div>();
    #[cfg(feature = "ssr")]
    dbg!("WARNING: calling use_element_hover on server");
    let is_hovered = use_element_hover(hover_ref);
    let input_ref = create_node_ref::<Input>();

    let (open, set_open) = create_signal(false);
    let filter = create_rw_signal(String::new());

    create_effect(move |_| {
        // log!("hovered: {}", is_hovered.get());
        set_open.set(is_hovered.get());
        filter.set(String::new());
    });

    let fast_option = create_rw_signal(None);

    let focus_searchbar = move || {
        let searchbar = input_ref.get()?;
        searchbar.focus().unwrap();
        Some(())
    };

    view! {
        <div node_ref=hover_ref class="menu-button">
            <button class="row center space" on:click=move |_| {
                focus_searchbar();
            }>
                {name}
                <div class="spacer"/>
                {Icons::ExpandArrow}
            </button>

            {move || {if open.get() {Some(view!{
                <div class="menu">
                <input node_ref=input_ref class="margin-small" type="text" placeholder="Suche..." prop:value=filter on:keypress=move |ev| {
                    if ev.key_code() == 13 {
                        if let Some(value) = fast_option.get_untracked() {
                            callback(value);
                            filter.set(String::new());
                        }
                    }
                } on:input=move |ev| {
                    filter.set(event_target_value(&ev))
                } />

                <ItemFilter callback fast_option items filter />
                </div>
            })} else {
                None
            }}
            }
        </div>
    }
}

#[component]
fn ItemFilter<
    V: Clone + 'static,
    F: Fn(V) + Copy + 'static,
    FastValue: SignalSet<Value = Option<V>> + Copy + 'static,
    Filter: SignalGet<Value = String> + Copy + 'static,
    I: SignalWith<Value = Vec<MenuItem<V>>> + Copy + 'static,
>(
    callback: F,
    fast_option: FastValue,
    items: I,
    filter: Filter,
) -> impl IntoView {
    view! {
        <div class="items">
            {move || {
                let filter = filter.get();
                let items: Vec<MenuItem<V>> = items.with(|items| items.iter().filter_map(move |item| if search_str(&item.name, &filter) {Some(item.clone())} else {None}).collect());

                fast_option.set(items.get(0).map(|item| item.clone_value()));

                items.iter().map(move |item| {
                    item.get_view(callback)
                }).collect_view()
            }}
        </div>
    }
}

// MenuItem

pub struct MenuItem<V> {
    value: V,
    name: String,
    icon: Option<String>,
}

impl<V: Clone + 'static> MenuItem<V> {
    pub fn new<S: Into<String>>(value: V, name: S) -> Self {
        Self {
            value,
            name: name.into(),
            icon: None,
        }
    }

    pub fn with_icon<S: Into<String>, I: Into<String>>(value: V, name: S, icon: I) -> Self {
        Self {
            value,
            name: name.into(),
            icon: Some(icon.into()),
        }
    }

    fn get_view<F: Fn(V) + 'static>(&self, callback: F) -> View {
        let value = self.value.clone();
        view! {
            <button class="row center" on:click=move |_| {
                callback(value.clone());
            }>
                {self.icon.clone().map(|icon| view! {
                    <Icon icon=&icon/>
                })}
                <span>
                    {self.name.clone()}
                </span>
            </button>
        }
        .into_view()
    }

    fn clone_value(&self) -> V {
        self.value.clone()
    }
}

impl<V: Clone> Clone for MenuItem<V> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            name: self.name.clone(),
            icon: self.icon.clone(),
        }
    }
}
