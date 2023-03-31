use data::Table;

use super::user::User;

#[derive(Table, Debug)]
pub struct Payment {
  #[primary]
  pub id: i64,
  pub name: String,
  #[foreign(User)]
  #[on_delete("restrict")]
  pub owner_id: i64,
}