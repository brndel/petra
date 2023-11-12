use leptos::*;
use leptos_router::A;

use crate::{
    api::{payment::Payment, category::Category, user::User},
    component::{amount::Amount, category::CategoryView, user::UserView, icon::Icons},
    util::{lang::Translate, calculated_amount::CalculatedAmount}, provider::{Provider, Me},
};

#[component]
pub fn PaymentView(payment: Payment) -> impl IntoView {
    let me_prov = Provider::<Me>::expect();
    let me = move || me_prov.get_single();
    
    let user_prov = Provider::<User>::expect();
    let owner = payment.owner.clone();
    let owner = move || user_prov.get(&owner);

    let category_prov = Provider::<Category>::expect();
    let categories = payment.categories.clone();
    let categories = move || category_prov.get_multiple(&categories);

    view! {
        {move ||
            match (me(), owner(), categories()) {
                (Some(me), Some(owner), Some(categories)) => Some({
                    let amount = CalculatedAmount::new(&me.id, &payment);
                    let name = payment.name.clone();
                    let id = payment.id.clone();
                    view! {
                        <A href={format!("?payment={}", id)} class="card payment">
                            <div class="row center space">
                                <span class="bold">
                                    {if payment.imported {Some(Icons::Imported)} else {None}}
                                    {name}
                                </span>
                                <Amount amount={amount.calculated_amount()} />
                            </div>
        
                            <div class="row center space">
                                <div class="row center">
                                    <span>{payment.timestamp.translate_default()}</span>
                                    <div class="row center">
                                    {
                                        categories.iter().map(|category| view! {<CategoryView category/>}).collect_view()
                                    }
                                    </div>
                                </div>
        
                                <div class="row center">
                                    {if amount.repay_amount != 0 {
                                        Some(Icons::Repay)
                                    } else {
                                        None
                                    }
                                    } 
                                    <UserView user={&owner} />
                                </div>
                            </div>
        
                        </A>
                    }
                }),
                _ => None,
            }
        }
    }
}
