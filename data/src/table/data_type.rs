use std::fmt::Display;

pub struct DataType {
  sql_type: &'static str,
  optional: bool,
}

impl Display for DataType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}{}",
      self.sql_type,
      if self.optional { "" } else { " NOT NULL" }
    )
  }
}

pub trait AsDataType {
  fn as_data_type() -> DataType;
}

impl AsDataType for i64 {
  fn as_data_type() -> DataType {
    DataType {
      sql_type: "INTEGER",
      optional: false,
    }
  }
}

impl AsDataType for String {
  fn as_data_type() -> DataType {
    DataType {
      sql_type: "TEXT",
      optional: false,
    }
  }
}

impl<T: AsDataType> AsDataType for Option<T> {
  fn as_data_type() -> DataType {
    let inner_type = T::as_data_type().sql_type;
    DataType {
      sql_type: inner_type,
      optional: false,
    }
  }
}
