use data::Table;
#[derive(Table)]
pub struct Category {
  #[primary]
  pub id: i64,
  pub name: String,
  pub icon: String
}