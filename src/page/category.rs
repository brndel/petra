use leptos::*;
use leptos_router::{use_params_map, use_query_map, Outlet, A};
use mensula_key::Key;

use crate::{
    api::category::{
        delete_category, delete_category_group, get_categories_in_group, get_category,
        get_category_group, update_category, update_category_group, Category, CategoryGroup, add_category_group, add_category,
    },
    component::{
        field::{
            category_group_field::CategoryGroupField, field::Field, icon_field::IconField,
            submit_field::SubmitField, text_field::TextField,
        },
        icon::Icon,
        response_builder::ResponseBuilder,
    },
    provider::Provider,
    util::reload_signal::ReloadSignal,
};

#[derive(Clone)]
struct CategoryGroupReload(ReloadSignal);

#[derive(Clone)]
struct CategoryReload(ReloadSignal);

#[component]
pub fn CategoryPage() -> impl IntoView {
    let category_group_reload = ReloadSignal::with_name("CategoryGroup");
    Provider::<CategoryGroup>::provide_with_reload(category_group_reload);

    provide_context(CategoryGroupReload(category_group_reload));
    provide_context(CategoryReload(ReloadSignal::with_name("Category")));

    let group_prov = Provider::<CategoryGroup>::expect();

    view! {
            <div class="side left col load-anim">
                <ResponseBuilder res={group_prov.resource()} builder={
                    |(category_groups, _)| view! {
                        {
                            category_groups.into_iter().map(|group| view! {<CategoryGroupButton group />}).collect_view()
                        }
                        <div class="spacer"/>
                        <A class="card" href="new">"New"</A>
                    }} />
            </div>

            <Outlet/>
        }
}

#[component]
fn CategoryGroupButton(group: CategoryGroup) -> impl IntoView {
    view! {
        <A href={Into::<String>::into(group.id)} class="row center card">
            <Icon icon={group.icon} />
            <span>{group.name}</span>
        </A>
    }
}

#[component]
pub fn CategoryPageEmpty() -> impl IntoView {
    view! {
        <div class="side right"/>
    }
}

#[component]
pub fn CategoryPageMain() -> impl IntoView {
    let params = use_params_map();
    let reload_signal = expect_context::<CategoryReload>().0;

    let categories = create_resource(
        move || (reload_signal.get(), params.get()),
        move |(_, params)| async move {
            let group_id = params
                .get("group")
                .ok_or(ServerFnError::ServerError("no group".to_string()))?
                .clone();

            let group: Key = group_id.into();

            Ok((get_categories_in_group(group.clone()).await?, group))
        },
    );

    view! {
        <main class="col load-anim">
            <ResponseBuilder res=categories builder=|(_, group)| {
                view! {
                    <h2>
                    {move || {
                        Provider::<CategoryGroup>::expect().get(&group).map(|group| group.name)
                    }}
                    </h2>
                }
            }/>
            <ResponseBuilder res=categories builder=|(categories, _)| view! {
                {
                    categories.into_iter().map(|category| view! {
                        <A href={format!("?category={}", category.id.as_ref())} class="card row center">
                            <Icon icon={category.icon}/>
                            <span>{category.name}</span>
                        </A>
                    }).collect_view()
                }
                <div class="spacer"/>
                <A class="card" href="?category=new">"New"</A>
            }/>
        </main>

        <CategoryDetails/>
    }
}

#[derive(Clone)]
enum Details {
    Category(Result<Category, Key>), // Abusing Result here
    Group(Option<CategoryGroup>),
}

#[component]
fn CategoryDetails() -> impl IntoView {
    let query = use_query_map();
    let params = use_params_map();

    let reload_signal = ReloadSignal::new();
    let reload = move || reload_signal.reload();

    let category = move || query.with(|query| query.get("category").cloned());
    let group = move || params.with(|params| params.get("group").cloned());

    let details = create_local_resource(
        move || (reload_signal.get(), category(), group()),
        |(_, category, group)| async move {

            let category = category.as_ref().map(String::as_str);
            let group = group.as_ref().map(String::as_str);

            let details = match (category, group) {
                (Some("new"), Some(group)) => {
                    Details::Category(Err(Key::from(group.to_string())))
                },
                (Some(category), _) => {
                    let category = get_category(Key::from(category.to_string())).await?;

                    Details::Category(Ok(category))
                },
                (_, Some("new")) => {
                    Details::Group(None)
                },
                (_, Some(group)) => {
                    let group = get_category_group(Key::from(group.to_string())).await?;

                    Details::Group(Some(group))
                },
                _ => {
                    return Ok(None);
                }
            };

            Ok(Some(details))
        },
    );

    view! {
        <div class="side right col load-anim">
            <ResponseBuilder res=details builder=move |data| {
                match data {
                    Some(Details::Category(category)) => Some({
                        let reload_signal = expect_context::<CategoryReload>().0;
                        let reload = reload_signal.subscribe(reload);

                        let id = category.as_ref().ok().map(|category| RwSignal::new(category.id.clone()));
                        let name = Field::new(category.as_ref().map(|c| c.name.clone()).unwrap_or_default());
                        let icon = Field::new(category.as_ref().map(|c| c.icon.clone()).unwrap_or_default());
                        let group = match &category {
                            Ok(category) => category.group.clone(),
                            Err(group) => group.clone(),
                        };
                        let group = Field::new(group);

                        view! {
                            <TextField signal=name/>
                            <IconField signal=icon/>
                            <CategoryGroupField signal=group/>

                            <div class="spacer"/>

                            <SubmitField
                                is_new=category.is_err()
                                field=(name, icon, group)
                                on_submit=move || {
                                    spawn_local(async move {
                                        let result;
                                        if let Some(id) = id {
                                            result = update_category(
                                                id.get_untracked(),
                                                name.get_untracked(),
                                                icon.get_untracked(),
                                                group.get_untracked()
                                            ).await;
                                        } else {
                                            result = add_category(
                                                name.get_untracked(),
                                                icon.get_untracked(),
                                                group.get_untracked()
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
                                            let id = id.get_untracked();
                                            let result = delete_category(id).await;
                                            if result.is_ok() {
                                                reload();
                                            }
                                        }
                                    });
                                }
                            />
                        }.into_view()
                    }),
                    Some(Details::Group(group)) => Some({
                        let reload_signal = expect_context::<CategoryGroupReload>().0;
                        let reload = reload_signal.subscribe(reload);

                        let name = group.as_ref().map(|group| group.name.clone()).unwrap_or_default();
                        let icon = group.as_ref().map(|group| group.icon.clone()).unwrap_or_default();

                        let id = group.as_ref().map(|group| RwSignal::new(group.id.clone()));
                        let name = Field::new(name);
                        let icon = Field::new(icon);

                        view! {
                            <TextField signal=name/>
                            <IconField signal=icon/>

                            <div class="spacer"/>

                            <SubmitField
                                is_new=group.is_none()
                                field=(name, icon)
                                on_submit=move || {
                                    spawn_local(async move {
                                        let result;
                                        if let Some(id) = id {
                                            result = update_category_group(
                                                id.get_untracked(),
                                                name.get_untracked(),
                                                icon.get_untracked()
                                            ).await;
                                        } else {
                                            result = add_category_group(
                                                name.get_untracked(),
                                                icon.get_untracked(),
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
                                            let result = delete_category_group(id.get_untracked()).await;
                                            if result.is_ok() {
                                                reload();
                                            }
                                        }
                                    });
                                }
                            />
                        }.into_view()
                    }),
                    None => None
                }
            } />
        </div>
    }
}
