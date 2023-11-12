use leptos::*;

use crate::component::{error::Error, loading::Loading};

#[component]
pub fn ResponseBuilder<
    T: 'static + Clone,
    F: 'static + Copy + FnOnce(T) -> V,
    V: 'static + IntoView,
    S: 'static + Clone,
>(
    res: Resource<S, Result<T, ServerFnError>>,
    builder: F,
) -> impl IntoView {
    view! {
        <Suspense fallback= move || view!{<Loading/>}>
            <ErrorBoundary fallback=move|error| view! {<Error error/>}>
            { move || {
                let data = res.get();

                data.map(move |data| data.map( move |data|
                    builder(data).into_view()
                ))

                }}
            </ErrorBoundary>
        </Suspense>
    }
}

// fn add_loaded_class(view: View) -> View {
//     view
//     // if is_server() {
//     //     return view;
//     // }

//     // log!("adding loaded to {:?}", view);

//     // match view {
//     //     View::Component(mut comp) => {
//     //         log!("component");
//     //         comp.children = comp.children.into_iter().map(add_loaded_class).collect();

//     //         comp.into_view()
//     //     }
//     //     View::Element(elem) => {
//     //         log!("element");
//     //         let elem = elem.into_html_element().class("loaded", true);

//     //         elem.into_view()
//     //     },
//     //     view @ View::Text(_) => {log!("text"); view},
//     //     view @ View::CoreComponent(mut comp) => {
//     //         log!("core");
//     //         if let CoreComponent::DynChild(dyn_child) = comp {
//     //         }
//     //     },
//     //     view @ View::Transparent(_) => {log!("transparent"); view},
//     //     view @ View::Suspense(_, _) => {log!("suspense"); view},

//     //     // _ => {
//     //     //     view
//     //     // }
//     // }
// }
