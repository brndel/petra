use mensula_key::Key;
use serde::{Serialize, Deserialize};

use crate::component::select_menu::MenuItem;


#[derive(Serialize, Deserialize, Clone)]
pub struct CategoryGroup {
    pub id: Key,
    pub name: String,
    pub icon: String,
    pub categories: Vec<Key>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: Key,
    pub name: String,
    pub icon: String,
    pub group: Key,
}


impl From<CategoryGroup> for MenuItem<Key> {
    fn from(value: CategoryGroup) -> Self {
        MenuItem::with_icon(value.id, value.name, value.icon)
    }
}

impl From<Category> for MenuItem<Key> {
    fn from(value: Category) -> Self {
        MenuItem::with_icon(value.id, value.name, value.icon)
    }
}

// impl IntoMenuItem for CategoryGroup {
//     type Value = Key;

//     fn value(self) -> Self::Value {
//         self.id
//     }

//     fn name(&self) -> String {
//         self.name.clone()
//     }

//     fn icon(&self) -> Option<String> {
//         Some(self.icon.clone())
//     }
// }


// impl IntoMenuItem for Category {
//     type Value = Key;

//     fn value(self) -> Self::Value {
//         self.id
//     }

//     fn name(&self) -> String {
//         self.name.clone()
//     }

//     fn icon(&self) -> Option<String> {
//         Some(self.icon.clone())
//     }
// }