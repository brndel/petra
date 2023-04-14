use data::Table;
use serde::Serialize;


#[derive(Table, Serialize)]
pub struct Category {
  #[primary]
  pub id: i64,
  pub name: String,
  pub icon: String,
  #[foreign(CategoryGroup)]
  pub group_id: i64,
}


#[derive(Table, Serialize)]
pub struct CategoryGroup {
  #[primary]
  pub id: i64,
  pub name: String,
  pub icon: String,
}
