use crate::api::category::Category;
use crate::api::rule::{get_rule, Rule, update_rule, add_rule, delete_rule};
use crate::component::field::category_field::CategoryField;
use crate::component::field::choice_field::ChoiceField;
use crate::component::field::field::Field;
use crate::component::field::submit_field::SubmitField;
use crate::component::field::text_field::{TextField, TextFieldStyle};
use crate::component::field::text_vec_field::TextVecField;
use crate::component::icon::{Icon, Icons};
use crate::component::response_builder::ResponseBuilder;
use crate::provider::Provider;
use crate::util::reload_signal::ReloadSignal;
use crate::util::search::search_str;
use leptos::*;
use leptos_router::{Outlet, A, use_params_map};
use mensula_key::Key;

#[component]
pub fn RulePage() -> impl IntoView {
    let reload_signal = ReloadSignal::new();
    Provider::<Rule>::provide_with_reload(reload_signal);
    Provider::<Category>::provide();

    provide_context(reload_signal);

    let rule_prov = Provider::<Rule>::expect();

    view! {
        <div class="side left col"/>

        <main class="col load-anim">
            <ResponseBuilder res={rule_prov.resource()} builder={
                |(rules, _)| {
                    let filter = RwSignal::new(String::new());


                    view! {
                        <div class="row center">
                            <div class="spacer">
                                <TextField signal=filter style=TextFieldStyle::Search/>
                            </div>
                            <A class="card" href="new">
                                {Icons::Add} "New"
                            </A>
                        </div>
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

    let reload_signal = expect_context::<ReloadSignal>();
    let reload = move || reload_signal.reload();

    let rule_id = move || params.with(|params| params.get("rule").cloned());

    let rule = create_local_resource(
        move || (reload_signal.get(), rule_id()),
        |(_, rule_id)| async move {
            if let Some(rule_id) = rule_id {
                if rule_id == "new" {
                    Ok(Some(None))
                } else {
                    let rule = get_rule(Key::from(rule_id)).await?;

                    Ok(Some(Some(rule)))
                }
            } else {
                Ok(None)
            }
        },
    );

    view! {
        <ResponseBuilder res={rule} builder=move |rule| match rule {
            Some(rule) => Some({
                let id = rule.as_ref().map(|rule| RwSignal::new(rule.id.clone()));
                let name = Field::new(rule.as_ref().map(|rule| rule.name.clone()).unwrap_or_default());
                let share_rule = Field::new(rule.as_ref().map(|rule| rule.share_rule.clone()).unwrap_or_default());
                let categories = Field::new(rule.as_ref().map(|rule| rule.categories.clone()).unwrap_or_default());
                let keywords = Field::new(rule.as_ref().map(|rule| rule.keywords.clone()).unwrap_or_default());

                view!{
                    <TextField signal=name/>
                    <ChoiceField signal=share_rule/>
                    <CategoryField signal=categories/>
                    <span>"Keywords"</span>
                    <TextVecField signal=keywords/>

                    <div class="spacer"/>

                    <SubmitField
                        is_new=rule.is_none()
                        field=(name, share_rule, categories, keywords)
                        on_submit=move || {
                            spawn_local(async move {
                                let result;
                                if let Some(id) = id {
                                    result = update_rule(
                                        id.get_untracked(),
                                        name.get_untracked(),
                                        share_rule.get_untracked(),
                                        keywords.get_untracked(),
                                        categories.get_untracked(),
                                    ).await;
                                } else {
                                    result = add_rule(
                                        name.get_untracked(),
                                        share_rule.get_untracked(),
                                        keywords.get_untracked(),
                                        categories.get_untracked(),
                                    ).await;
                                }
                                if result.is_ok() {
                                    reload();
                                }
                            });
                        }
                        on_delete=move || {
                            spawn_local(async move {
                                if let Some(id) = id {
                                    let result = delete_rule(id.get_untracked()).await;
                                    if result.is_ok() {
                                        reload();
                                    }
                                }
                            })
                        }
                    />
                }
            }),
            None => None
        }/>
    }
}