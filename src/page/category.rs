use leptos::*;
use leptos_router::{use_params_map, use_query_map, Outlet, A};
use mensula_key::Key;

use crate::{
    api::category::{
        get_categories_in_group, get_category, get_category_group, update_category,
        update_category_group, Category, CategoryGroup,
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
                |(category_groups, _)|
                category_groups.into_iter().map(|group| view! {<CategoryGroupButton group />}).collect_view()} />
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
            <ResponseBuilder res=categories builder=|(categories, _)| {
                categories.into_iter().map(|category| view! {
                    <A href={format!("?selected={}", category.id.as_ref())} class="card row center">
                        <Icon icon={category.icon}/>
                        <span>{category.name}</span>
                    </A>
                }).collect_view()
            }/>
        </main>

        <CategoryDetails/>
    }
}

#[derive(Clone)]
enum Details {
    Category(Category),
    Group(CategoryGroup),
}

#[component]
fn CategoryDetails() -> impl IntoView {
    let query = use_query_map();
    let params = use_params_map();

    let reload_signal = ReloadSignal::new();
    let reload = move || reload_signal.reload();

    let details = create_local_resource(
        move || (reload_signal.get(), query.get(), params.get()),
        |(_, query, params)| async move {
            if let Some(category) = query.get("selected") {
                let category = get_category(category.clone().into()).await;

                category.map(Details::Category).map(Some)
            } else if let Some(group) = params.get("group") {
                let group = get_category_group(group.clone().into()).await;

                group.map(Details::Group).map(Some)
            } else {
                Ok(None)
            }
        },
    );

    view! {
        <div class="side right col load-anim">
            <ResponseBuilder res=details builder=move |data| {
                match data {
                    Some(Details::Category(category)) => Some({
                        let reload_signal = expect_context::<CategoryReload>().0;
                        let reload = reload_signal.subscribe(reload);

                        let name = Field::new(&category.name);
                        let icon = Field::new(&category.icon);
                        let group = Field::new(&category.group);

                        view! {
                            <TextField signal=name/>
                            <IconField signal=icon/>
                            <CategoryGroupField signal=group/>

                            <div class="spacer"/>

                            <SubmitField
                                field=(name, icon, group)
                                on_submit=move || {
                                    let id = category.id.clone();
                                    spawn_local(async move {
                                        let result = update_category(
                                            id,
                                            name.get_untracked(),
                                            icon.get_untracked(),
                                            group.get_untracked().into()
                                        ).await;
                                        if result.is_ok() {
                                            reload();
                                        }
                                    });
                                }
                            />
                        }.into_view()
                    }),
                    Some(Details::Group(group)) => Some({
                        let reload_signal = expect_context::<CategoryGroupReload>().0;
                        let reload = reload_signal.subscribe(reload);

                        let name = Field::new(&group.name);
                        let icon = Field::new(&group.icon);

                        view! {
                            <TextField signal=name/>
                            <IconField signal=icon/>

                            <div class="spacer"/>

                            <SubmitField
                                field=(name, icon)
                                on_submit=move || {
                                    let id = group.id.clone();
                                    spawn_local(async move {
                                        let result = update_category_group(
                                            id,
                                            name.get_untracked(),
                                            icon.get_untracked()
                                        ).await;
                                        if result.is_ok() {
                                            reload();
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
