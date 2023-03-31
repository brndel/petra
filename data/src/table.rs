use sqlite::Statement;

use crate::Column;

pub trait Table where Self: Sized {
  fn table_name() -> &'static str;
  fn primary_name() -> &'static str;
  fn get_columns() -> Vec<Column>;
  fn read(statement: &sqlite::Statement) -> sqlite::Result<Self>;

  fn get_primary(&self) -> i64;
}

pub trait Insertable<T: Table> {
  fn get_column_names() -> &'static [&'static str];
  fn get_placeholder_names() -> &'static [&'static str];

  fn bind(self, statement: &mut Statement) -> sqlite::Result<()>;
}

pub trait Link<T: Table> {
  fn get_name() -> &'static str;
}