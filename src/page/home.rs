use leptos::*;

use crate::{api::{payment::calculate_all_amounts, user::User}, provider::{Provider, Me}, component::{response_builder::ResponseBuilder, user::UserView, amount::Amount, icon::Icons}};


#[component]
pub fn HomePage() -> impl IntoView {
    Provider::<User>::provide();
    
    let amount = create_resource(||(), |_| calculate_all_amounts());

    let user_prov = Provider::<User>::expect();
    let me_prov = Provider::<Me>::expect();

    view! {
        <main class="col start load-anim">
            <ResponseBuilder res=amount builder=move |data| view! {
                <div class="card col stretch">
                    {me_prov.get_single().map(|me| view! {<UserView user=&me big=true/>})}

                    <div class="card row center space">
                        <span>"Gesamtausgaben"</span>
                        <Amount amount=data.own.user_amount/>
                    </div>

                    <div class="card row center space">
                        <span>"Rückzahlung"</span>
                        {Icons::Repay}
                        <Amount amount=data.own.repay_amount/>
                    </div>

                    <div class="card row center space">
                        <span>"Summe"</span>
                        {Icons::ArrowRight}
                        <Amount amount=data.own.calculated_amount()/>
                    </div>
                </div>

                <h2>"Andere"</h2>

                <div class="row">
                    {data.others.into_iter().map(|(user, amount)| view! {
                        <div class="card row center">
                            {user_prov.get(&user).map(|user| view! {<UserView user=&user big=true/>})}

                            <div class="card row center space">
                                <span>"Rückzahlung"</span>
                                {Icons::Repay}
                                <Amount amount=amount.repay_amount/>
                            </div>
                        </div>
                    }).collect_view()}
                </div>
            }/>
        </main>
    }
}
