use crate::api::category::Category;
use crate::api::rule::{get_rule, Rule};
use crate::component::field::choice_field::ChoiceField;
use crate::component::field::field::Field;
use crate::component::field::submit_field::SubmitField;
use crate::component::field::text_field::{TextField, TextFieldStyle};
use crate::component::icon::Icon;
use crate::component::response_builder::ResponseBuilder;
use crate::provider::Provider;
use crate::util::reload_signal::ReloadSignal;
use crate::util::search::search_str;
use leptos::*;
use leptos_router::{Outlet, A, use_params_map};

#[component]
pub fn RulePage() -> impl IntoView {
    Provider::<Rule>::provide();
    Provider::<Category>::provide();

    let rule_prov = Provider::<Rule>::expect();

    view! {
        <div class="side left col"/>

        <main class="col load-anim">
            <ResponseBuilder res={rule_prov.resource()} builder={
                |(rules, _)| {
                    let filter = RwSignal::new(String::new());


                    view! {
                        <TextField signal=filter style=TextFieldStyle::Search/>
                        {move || {
                            let filter = filter.get().to_lowercase();
                            rules.iter().filter_map(
                                move |rule| if search_str(&rule.name, &filter) {
                                    Some(view! {<RuleButton rule=rule.clone() />})
                                } else {
                                    None
                                }
                            )
                        }.collect_view()}
                    }
                }}
                />
        </main>

        <div class="side right col load-anim">
            <Outlet/>
        </div>
    }
}

#[component]
pub fn RuleButton(rule: Rule) -> impl IntoView {
    let category_prov = Provider::<Category>::expect();

    view! {
        <A href={Into::<String>::into(rule.id)} class="col card">
            <div class="row center">
                <span>{rule.name}</span>
                {move || rule.categories.iter().map(|id| category_prov.get(id).map(|category| view!{<Icon icon={category.icon}/>})).collect_view()}
            </div>
            <div class="row center">
                {rule.keywords.iter().map(|kw| view! {<span class="card">"\""{kw.clone()}"\""</span>}).collect_view()}
            </div>
        </A>
    }
}

#[component]
pub fn RulePageDetails() -> impl IntoView {
    let params = use_params_map();

    let reload_signal = ReloadSignal::new();
    let reload = move || reload_signal.reload();


    let payment = create_local_resource(
        move || (reload_signal.get(), params.get()),
        |(_, query)| async move {
            let id = query.get("rule").cloned();
            fetch_rule(id).await
        },
    );

    view! {
        <ResponseBuilder res={payment} builder=move |payment| match payment {
            Some(rule) => Some({
                let name = Field::new(&rule.name);
                let share_rule = Field::new(&rule.share_rule);
                // let

                view!{
                    <TextField signal=name/>
                    <ChoiceField signal=share_rule/>

                    // TODO
                    <span>"Mehr Optionen kommen bald"</span>

                    <div class="spacer"/>

                    <SubmitField
                        field=(name, share_rule)
                        on_submit=move || {
                            let id = rule.id.clone();
                        }
                    />
                }
            }),
            None => None
        }/>
    }
}

async fn fetch_rule(id: Option<String>) -> Result<Option<Rule>, ServerFnError> {
    let id = match id {
        Some(id) => id,
        None => return Ok(None),
    };

    get_rule(id.into()).await.map(Some)
}
