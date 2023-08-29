use leptos::*;
use leptos_meta::{Link, Stylesheet, Title};
use leptos_router::{Router, Routes, Route};

use crate::page::Home;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,

        <Stylesheet href="/pkg/petra.css" />

        <Link rel="shortcut icon" href="favicon.png" type_="image/png"/>

        <Title text="Petra"/>

        <Router>
        <header>
        <nav class="row">
            <a href="/">Petra</a>
            <div class="spacer"/>
            <a href="/user">User</a>
        </nav>
        </header>
        
        <main>
            <Routes>
                <Route path="/" view=Home />
            </Routes>
        </main>
        </Router>
    }
}
