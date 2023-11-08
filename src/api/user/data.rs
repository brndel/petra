use mensula_key::Key;
use serde::{Deserialize, Serialize};

use crate::component::select_menu::MenuItem;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Key,
    pub name: String,
    pub display_name: String,
}

impl From<User> for MenuItem<Key> {
    fn from(value: User) -> Self {
        MenuItem::new(value.id, value.display_name)
    }
}
