use leptos::{ServerFnError, server};
use mensula_key::Key;

pub use super::data::*;

#[cfg(feature = "ssr")]
use super::server;

#[derive(Debug)]
pub struct CategoryFetchError;

#[derive(Debug)]
pub struct CategoryAddError;

impl From<CategoryFetchError> for ServerFnError {
    fn from(_: CategoryFetchError) -> Self {
        ServerFnError::ServerError("could not get category".into())
    }
}

impl From<CategoryAddError> for ServerFnError {
    fn from(_: CategoryAddError) -> Self {
        ServerFnError::ServerError("could not add category".into())
    }
}

// Get Lists

#[server]
pub async fn get_category_groups() -> Result<Vec<CategoryGroup>, ServerFnError> {
    server::get_category_groups().map_err(Into::into)
}

#[server]
pub async fn get_categories() -> Result<Vec<Category>, ServerFnError> {
    server::get_categories().map_err(Into::into)
}

#[server]
pub async fn get_categories_in_group(group_id: Key) -> Result<Vec<Category>, ServerFnError> {
    server::get_categories_in_group(group_id).map_err(Into::into)
}

// Get single

#[server]
pub async fn get_category(id: Key) -> Result<Category, ServerFnError> {
    server::get_category(id).map_err(Into::into)
}

#[server]
pub async fn get_category_group(id: Key) -> Result<CategoryGroup, ServerFnError> {
    server::get_category_group(id).map_err(Into::into)
}

// Add single

#[server]
pub async fn add_category(name: String, icon: String, group: Key) -> Result<Key, ServerFnError> {
    server::insert_category(None, name, icon, group).map_err(Into::into)
}

#[server]
pub async fn add_category_group(name: String, icon: String) -> Result<Key, ServerFnError> {
    server::insert_category_group(None, name, icon).map_err(Into::into)
}

// Update

#[server]
pub async fn update_category(
    id: Key,
    name: String,
    icon: String,
    group: Key,
) -> Result<Key, ServerFnError> {
    server::insert_category(Some(id), name, icon, group).map_err(Into::into)
}

#[server]
pub async fn update_category_group(
    id: Key,
    name: String,
    icon: String,
) -> Result<Key, ServerFnError> {
    server::insert_category_group(Some(id), name, icon).map_err(Into::into)
}

// Delete

#[server]
pub async fn delete_category(
    id: Key
) -> Result<(), ServerFnError> {
    if server::delete_category(id) {
        Ok(())
    } else {
        Err(ServerFnError::ServerError("Could not delete".to_string()))
    }
}

#[server]
pub async fn delete_category_group(
    id: Key
) -> Result<(), ServerFnError> {
    if server::delete_category_group(id) {
        Ok(())
    } else {
        Err(ServerFnError::ServerError("Could not delete".to_string()))
    }
}