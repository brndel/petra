use leptos::*;

#[cfg(feature = "ssr")]
use super::server;

pub use super::data::*;

pub struct UserFetchError;

impl From<UserFetchError> for ServerFnError {
    fn from(_: UserFetchError) -> Self {
        ServerFnError::ServerError("could not get user".into())
    }
}

#[server]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    server::get_all_users().map_err(Into::into)
}

#[server]
pub async fn me() -> Result<User, ServerFnError> {
    let id = crate::auth::get_user().await?;

    server::get_user(id).map_err(Into::into)
}

#[cfg(feature = "ssr")]
pub fn get_user_by_name(name: String) -> Option<server::User> {
    server::get_user_by_name(name)
}