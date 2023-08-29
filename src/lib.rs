mod app;
mod page;

#[cfg(feature="web")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;
    use leptos::*;

    leptos::mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}