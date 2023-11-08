use mensula::{
    query::{Ordering, SelectQuery},
    Table,
};
use mensula_key::Key;

use crate::db::get_db;

use super::{
    api::{CategoryAddError, CategoryFetchError},
    data::{Category as ResponseCategory, CategoryGroup as ResponseCategoryGroup},
};

#[derive(Table)]
pub struct Category {
    #[primary]
    id: Key,
    name: String,
    icon: String,
    // group is a reserved keyword in SQL, so this needs to be called group_id
    #[foreign(CategoryGroup)]
    group_id: Key,
}

#[derive(Table)]
pub struct CategoryGroup {
    #[primary]
    id: Key,
    name: String,
    icon: String,
}

impl From<(CategoryGroup, Vec<Key>)> for ResponseCategoryGroup {
    fn from(value: (CategoryGroup, Vec<Key>)) -> Self {
        let (group, categories) = value;
        ResponseCategoryGroup {
            id: group.id,
            name: group.name,
            icon: group.icon,
            categories,
        }
    }
}

impl From<Category> for ResponseCategory {
    fn from(value: Category) -> Self {
        Self {
            id: value.id,
            name: value.name,
            icon: value.icon,
            group: value.group_id,
        }
    }
}

pub fn get_category_groups() -> Result<Vec<ResponseCategoryGroup>, CategoryFetchError> {
    let db = get_db();

    let groups = SelectQuery::new()
        .order_by(CategoryGroup::name(), Ordering::Ascending)
        .get_all::<CategoryGroup>(&db)
        .ok_or(CategoryFetchError)?;

    let mut group_vec = Vec::new();

    for group in groups {
        let categories = SelectQuery::new()
            .filter(Category::group_id().eq(group.id.clone()))
            .order_by(Category::name(), Ordering::Ascending)
            .get_all::<Key>(&db)
            .ok_or(CategoryFetchError)?;

        group_vec.push((group, categories).into());
    }

    Ok(group_vec)
}

pub fn get_categories() -> Result<Vec<ResponseCategory>, CategoryFetchError> {
    let db = get_db();

    let categories = SelectQuery::new()
        .order_by(Category::name(), Ordering::Ascending)
        .get_all::<Category>(&db)
        .ok_or(CategoryFetchError)?;

    let categories = categories.into_iter().map(Into::into).collect();

    Ok(categories)
}

pub fn get_categories_in_group(group_id: Key) -> Result<Vec<ResponseCategory>, CategoryFetchError> {
    let db = get_db();

    let categories = SelectQuery::new()
        .filter(Category::group_id().eq(group_id))
        .order_by(Category::name(), Ordering::Ascending)
        .get_all::<Category>(&db);

    categories
        .map(|categories| categories.into_iter().map(Into::into).collect())
        .ok_or(CategoryFetchError)
}

pub fn get_category_group(id: Key) -> Result<ResponseCategoryGroup, CategoryFetchError> {
    let db = get_db();

    let group = db
        .get::<CategoryGroup>(id.clone())
        .ok_or(CategoryFetchError)?;

    let categories = SelectQuery::new()
        .filter(Category::group_id().eq(id))
        .get_all::<Key>(&db)
        .ok_or(CategoryFetchError)?;

    Ok((group, categories).into())
}

pub fn get_category(id: Key) -> Result<ResponseCategory, CategoryFetchError> {
    get_db()
        .get::<Category>(id)
        .ok_or(CategoryFetchError)
        .map(Into::into)
}

pub fn insert_category(
    id: Option<Key>,
    name: String,
    icon: String,
    group: Key,
) -> Result<Key, CategoryAddError> {
    get_db()
        .insert(Category {
            id: id.unwrap_or_else(Key::new),
            name,
            icon,
            group_id: group,
        })
        .ok_or(CategoryAddError)
}

pub fn insert_category_group(
    id: Option<Key>,
    name: String,
    icon: String,
) -> Result<Key, CategoryAddError> {
    get_db()
        .insert(CategoryGroup {
            id: id.unwrap_or_else(Key::new),
            name,
            icon,
        })
        .ok_or(CategoryAddError)
}
