use std::fmt::Display;

use crate::db::get_db;
use leptos::ServerFnError;
use mensula::{Table, query::SelectQuery};
use mensula_key::Key;

use super::{data::User as ResponseUser, api::UserFetchError};

#[derive(Table)]
pub struct User {
    #[primary]
    pub id: Key,
    pub name: String,
    pub display_name: String,
    password_hash: String,
}

impl From<User> for ResponseUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            name: value.name,
            display_name: value.display_name,
        }
    }
}

pub fn get_all_users() -> Result<Vec<ResponseUser>, UserFetchError> {
    let db = get_db();
    let users = SelectQuery::new().order_by(User::display_name(), mensula::query::Ordering::Ascending).get_all::<User>(&db).ok_or(UserFetchError)?;

    let users = users.into_iter().map(Into::into).collect();

    Ok(users)
}

pub fn get_user(id: Key) -> Result<ResponseUser, UserFetchError> {
    let user = get_db().get::<User>(id).ok_or(UserFetchError)?;

    Ok(user.into())
}

pub fn get_user_by_name(name: String) -> Option<User> {
    let db = get_db();
    
    SelectQuery::new()
        .filter(User::name().eq(name.to_owned()))
        .get_first(&db)
}

pub fn add_user(name: String, display_name: String, password: String) -> Result<Key, UserCreateError> {
    let db = get_db();

    let user = User::create(name, display_name, password)?;

    db.insert(user).ok_or(UserCreateError::Database)
}

#[derive(Debug)]
pub enum UserCreateError {
    InvalidName,
    InvalidDisplayname,
    InvalidPassword,
    Database,
}

impl Display for UserCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserCreateError::InvalidName => write!(f, "Username needs to have at least 2 characters and only contain lowercase letters, numbers and underscores"),
            UserCreateError::InvalidDisplayname => write!(f, "Display name needs to have at least 2 characters"),
            UserCreateError::InvalidPassword => write!(f, "Password needs to have at least 8 characters"),
            UserCreateError::Database => write!(f, "Could not insert user into database"),
        }
    }
}

impl From<UserCreateError> for ServerFnError {
    fn from(value: UserCreateError) -> Self {
        Self::ServerError(value.to_string())
    }
}

impl User {
    pub fn create(
        name: String,
        display_name: String,
        password: String,
    ) -> Result<User, UserCreateError> {
        Self::check_name(&name)?;
        Self::check_display_name(&display_name)?;
        Self::check_password(&password)?;

        let id = Key::new();
        let password_hash = Self::hash(id.as_ref(), &name, &password);

        Ok(Self {
            id,
            name,
            display_name,
            password_hash,
        })
    }

    pub fn authenticate(&self, password: &str) -> bool {
        let password_hash = Self::hash(self.id.as_ref(), &self.name, &password);

        password_hash == self.password_hash
    }

    fn hash(id: &str, name: &str, password: &str) -> String {
        sha256::digest(format!("{}|{}|{}", id, name, password))
    }

    pub fn check_name(name: &str) -> Result<(), UserCreateError> {
        if name.len() >= 2
            && name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            Ok(())
        } else {
            Err(UserCreateError::InvalidName)
        }
    }

    pub fn check_display_name(display_name: &str) -> Result<(), UserCreateError> {
        if display_name.len() >= 2 {
            Ok(())
        } else {
            Err(UserCreateError::InvalidDisplayname)
        }
    }

    pub fn check_password(password: &str) -> Result<(), UserCreateError> {
        if password.len() >= 8 {
            Ok(())
        } else {
            Err(UserCreateError::InvalidPassword)
        }
    }
}
