use leptos::*;

#[component]
pub fn Amount(amount: i64) -> impl IntoView {
    view! {
        <span class={format!("amount {}", if amount.is_positive() {"positive"} else if amount.is_negative() {"negative"} else {""})}>
            {to_amount_string(amount)}
        </span>
    }
}

fn to_amount_string(value: i64) -> String {
    format!("{:+}.{:02}", value / 100, value.abs() % 100)
}
