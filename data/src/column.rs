use crate::{DataType, Modifier};

pub struct Column {
  pub name: &'static str,
  pub data_type: DataType,
  pub modifier: Modifier,
}

impl Column {
  pub const fn new(name: &'static str, data_type: DataType, modifier: Modifier) -> Self {
    Self {
      name,
      data_type,
      modifier,
    }
  }
}

impl ToString for Column {
  fn to_string(&self) -> String {
    format!(
      "{} {}{}{}",
      self.name,
      self.data_type.sql_type,
      if self.data_type.optional {
        ""
      } else {
        " NOT NULL"
      },
      self.modifier.to_string(),
    )
  }
}
