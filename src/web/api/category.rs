use data::PrimKey;
use serde::Deserialize;

use crate::{
    tables::category::{Category, CategoryGroup, CategoryInsert, CategoryGroupInsert},
    Error, Request,
};

use super::serialize;

pub fn get_categories(request: &Request) -> Result<String, Error> {
    let categories = request.database.get_all::<Category>();

    serialize(&categories)
}

pub fn get_category_groups(request: &Request) -> Result<String, Error> {
    let groups = request.database.get_all::<CategoryGroup>();

    serialize(&groups)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddCategoryQuery {
    name: String,
    icon: String,
    group_id: PrimKey,
}

pub fn add_category(request: &Request) -> Result<String, Error> {
    let query: AddCategoryQuery = serde_json::from_str(&request.body)
        .map_err(|_| Error::BadRequest("could not deserialize request".to_string()))?;

    request.database.insert(CategoryInsert {
        name: query.name,
        icon: query.icon,
        group_id: query.group_id,
    }).ok_or(Error::Database)?;

    Ok("null".into())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddCategoryGroupQuery {
    name: String,
    icon: String,
}

pub fn add_category_group(request: &Request) -> Result<String, Error> {
    let query: AddCategoryGroupQuery = serde_json::from_str(&request.body)
        .map_err(|_| Error::BadRequest("could not deserialize request".to_string()))?;

    request.database.insert(CategoryGroupInsert {
        name: query.name,
        icon: query.icon,
    }).ok_or(Error::Database)?;

    Ok("null".into())
}
