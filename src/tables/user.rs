use data::Table;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Table)]
pub struct User {
  #[primary]
  pub id: i64,
  #[unique]
  pub name: String,
  pub display_name: String,
  pub auth_hash: String,
}
