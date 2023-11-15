use leptos::*;
use leptos_router::{use_query_map, A};
use mensula_key::Key;

use crate::{
    api::user::{get_user, User, add_user},
    component::{
        field::{
            field::Field,
            submit_field::SubmitField,
            text_field::{TextField, TextFieldStyle},
        },
        response_builder::ResponseBuilder,
        user::UserView, icon::Icons,
    },
    provider::Provider,
    util::reload_signal::ReloadSignal,
};

#[component]
pub fn UserPage() -> impl IntoView {
    let reload_signal = ReloadSignal::new();
    Provider::<User>::provide_with_reload(reload_signal);

    provide_context(reload_signal);

    let user_provider = Provider::<User>::expect();

    view! {
        <div class="side left col"/>


        <main class="col load-anim">
            <ResponseBuilder res={user_provider.resource()} builder=move |(users, _)| {

                view! {
                    {users.into_iter().map(|user| view!{<UserButton user/>}).collect_view()}
                    <div class="spacer"/>
                    <A class="card" href="?user=new">
                        {Icons::Add} "New"
                    </A>
                }
            }/>
        </main>

        <div class="side right col">
            <UserDetails/>
        </div>
    }
}

#[component]
fn UserButton(user: User) -> impl IntoView {
    view! {
        <A class="card row center" href=format!("?user={}", user.id)>
            <UserView user=&user/>
            <div class="col">
                <span>{user.display_name}</span>
                <span class="light">{user.name}</span>
            </div>
        </A>
    }
}

#[component]
fn UserDetails() -> impl IntoView {
    let query = use_query_map();
    let reload_signal = expect_context::<ReloadSignal>();

    let reload = move || reload_signal.reload();

    let user_id = move || query.with(|query| query.get("user").cloned());

    let user = create_resource(
        move || (reload_signal.get(), user_id()),
        |(_, user_id)| async move {
            match user_id.as_ref().map(String::as_str) {
                Some("new") => Ok::<_, ServerFnError>(Some(None)),
                Some(user_id) => {
                    let user = get_user(Key::from(user_id.to_string())).await?;

                    Ok(Some(Some(user)))
                }
                None => Ok(None),
            }
        },
    );

    view! {
        <ResponseBuilder res=user builder=move |user| {
            match user {
                Some(None) => Some({
                    let name = Field::new(String::new());
                    let display_name = Field::new(String::new());
                    let password = Field::new(String::new());
                    let password_repeat = Field::new(String::new());

                    view! {
                        <label>"Anzeigename"</label>
                        <TextField signal=display_name/>
                        <label>"Nutzername"</label>
                        <TextField signal=name/>
                        <label>"Passwort"</label>
                        <TextField signal=password style=TextFieldStyle::Password/>
                        <label>"Passwort wiederholen"</label>
                        <TextField signal=password_repeat style=TextFieldStyle::Password/>

                        <div class="spacer"/>

                        {move || if name.with(|name| name.len() >= 2) {None} else {Some(view! {
                            <span class="card error">
                                "Nutzername muss mindestens 2 Buchstaben lang sein"
                            </span>
                        })}}

                        {move || if display_name.with(|name| name.len() >= 2) {None} else {Some(view! {
                            <span class="card error">
                                "Anzeigename muss mindestens 2 Buchstaben lang sein"
                            </span>
                        })}}

                        {move || if password.with(|pw| pw.len() >= 8) {None} else {Some(view! {
                            <span class="card error">
                                "Passwörter muss mindestens 8 Buchstaben lang sein"
                            </span>
                        })}}

                        {move || if password.get() == password_repeat.get() {None} else {Some(view! {
                            <span class="card error">
                                "Passwörter stimmen nicht überein"
                            </span>
                        })}}

                        <SubmitField
                            is_new=true
                            field=(name, display_name, password, password_repeat)
                            on_submit=move || {
                                if name.with_untracked(|name| name.len() >= 2)
                                && display_name.with_untracked(|name| name.len() >= 2)
                                && password.with_untracked(|pw| pw.len() >= 8)
                                && password.get_untracked() == password_repeat.get_untracked() {
                                    spawn_local(async move {
                                        let result = add_user(name.get_untracked(), display_name.get_untracked(), password.get_untracked()).await;
    
                                        if result.is_ok() {
                                            reload();
                                        }
                                    })
                                }
                            }
                            on_delete=||()
                        />
                    }
                }.into_view()),
                Some(Some(_)) => Some({
                    view!{
                        "User bearbeiten geht gerade noch nicht"
                    }
                }.into_view()),
                None => None,
            }
        } />
    }
}
