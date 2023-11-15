use leptos::*;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{Route, Router, Routes, A};

use crate::{
    component::{
        icon::Icons,
        loading::Loading,
        user::UserView,
    },
    page::{
        add::AddPage,
        category::{CategoryPage, CategoryPageEmpty, CategoryPageMain},
        home::HomePage,
        payment::{PaymentPage, PaymentPageMain},
        rule::{RulePage, RulePageDetails}, user::UserPage,
    },
    provider::{Me, Provider},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    Provider::<Me>::provide();

    view! {
        <Stylesheet href="/pkg/petra.css" />

        <Link rel="shortcut icon" href="/favicon.png" type_="image/png"/>
        <Link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.2/css/all.min.css"/>

        <Title text="Petra"/>

        <Router>
        <header>
                <nav class="row center">
                <A class="card row center" href="/"> {Icons::Home} "Petra"</A>
                <div class="spacer"/>
                <A class="card row center" href="/rule"> {Icons::Rule} "Regeln"</A>
                <A class="card row center" href="/category"> {Icons::Category} "Kategorien"</A>
                <A class="card row center" href="/payment"> {Icons::Payment} "Zahlungen"</A>
                <A class="card row center" href="/add"> {Icons::AddPayment} "Eintragen"</A>
                <A href="/user">
                    <Suspense fallback=||view!{<Loading/>}>
                        {|| {
                            let me = Provider::<Me>::expect().get_single();

                            match &me {
                                Some(user) => view!{ <UserView user/> }.into_view(),
                                None => ().into_view()
                            }
                        }}
                    </Suspense>
                </A>
                </nav>
            </header>

            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/payment" view=PaymentPage>
                    <Route path="" view=Empty/>
                    <Route path=":month" view=PaymentPageMain/>
                </Route>
                <Route path="/category" view=CategoryPage>
                    <Route path="" view=CategoryPageEmpty/>
                    <Route path=":group" view=CategoryPageMain/>
                </Route>
                <Route path="/add" view=AddPage/>
                <Route path="/rule" view=RulePage>
                    <Route path="" view=Empty/>
                    <Route path=":rule" view=RulePageDetails/>
                </Route>

                <Route path="/user" view=UserPage>
                </Route>

                <Route path="/*" view=NotFound/>
            </Routes>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <main class="col center start">
            <h1>"Wie bist du denn hier gelandet?"</h1>
            <a class="card" href="/">"Zurück in die Sicherheit"</a>
        </main>
    }
}

#[component]
fn Empty() -> impl IntoView {
    ()
}
