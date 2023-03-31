pub struct DataType {
  pub sql_type: &'static str,
  pub optional: bool,
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
    let inner_type = T::as_data_type();
    DataType {
      sql_type: inner_type.sql_type,
      optional: true,
    }
  }
}