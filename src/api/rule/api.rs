use leptos::{ServerFnError, server};
use mensula_key::Key;

pub use super::data::*;

#[cfg(feature = "ssr")]
use super::server;

#[derive(Debug)]
pub struct RuleFetchError;
impl From<RuleFetchError> for ServerFnError {
    fn from(_: RuleFetchError) -> Self {
        ServerFnError::ServerError("could not get rule".to_owned())
    }
}

#[derive(Debug)]
pub struct RuleInsertError;
impl From<RuleInsertError> for ServerFnError {
    fn from(_: RuleInsertError) -> Self {
        ServerFnError::ServerError("could not insert rule".to_owned())
    }
}

#[server]
pub async fn get_rules() -> Result<Vec<Rule>, ServerFnError> {
    server::get_rules().map_err(Into::into)
}

#[server]
pub async fn get_rule(id: Key) -> Result<Rule, ServerFnError> {
    server::get_rule(id.clone()).map_err(Into::into)
}

#[server]
pub async fn add_rule(
    name: String,
    shared: ShareRule,
    keywords: Vec<String>,
    categories: Vec<Key>,
) -> Result<Key, ServerFnError> {
    server::insert_rule(None, name, shared, keywords, categories).map_err(Into::into)
}

#[server]
pub async fn update_rule(
    id: Key,
    name: String,
    shared: ShareRule,
    keywords: Vec<String>,
    categories: Vec<Key>,
) -> Result<Key, ServerFnError> {
    server::insert_rule(Some(id), name, shared, keywords, categories).map_err(Into::into)
}

#[server]
pub async fn delete_rule(
    id: Key
) -> Result<(), ServerFnError> {
    if server::delete_rule(id) {
        Ok(())
    } else {
        Err(ServerFnError::ServerError("Could not delete".to_string()))
    }
}