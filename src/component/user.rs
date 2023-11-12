use leptos::*;

use crate::api::user::User;

#[component]
pub fn UserView<'a>(user: &'a User, #[prop(optional)] big: bool) -> impl IntoView {
    let color = get_color(&user.name);
    view! {
        <div class="user-profile" class:big=big style={format!("background-color: {color};")}>
            <span>{user.name.clone()}</span>
        </div>
    }
}

fn get_color(name: &str) -> String {
    let mut hash: u32 = 0x6d2b79f5;

    for c in name.chars() {
        hash = ((c as u32) + (hash << 5)) ^ hash;
    }

    let hue = hash % 360;

    return format!("hsl({}, 80%, 40%)", hue);
}

