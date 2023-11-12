pub mod app;
pub mod page;
pub mod util;
pub mod component;
pub mod api;
pub mod provider;

#[cfg(feature = "ssr")]
pub mod db;
#[cfg(feature = "ssr")]
pub mod auth;
#[cfg(feature = "ssr")]
pub mod cli;

#[cfg(feature="hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;
    use leptos::*;

    leptos::mount_to_body(move || {
        view! { <App/> }
    });
}