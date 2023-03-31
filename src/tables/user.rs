use data::Table;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Table, Debug)]
pub struct User {
  #[primary]
  pub id: i64,
  // #[unique]
  pub name: String,
  pub display_name: String,
}
