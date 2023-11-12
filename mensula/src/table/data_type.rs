use mensula_key::Key;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct DataType {
    pub data_type: DataTypeKind,
    pub optional: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum DataTypeKind {
    Integer,
    Real,
    Text,
    Blob,
}

impl AsRef<str> for DataTypeKind {
    fn as_ref(&self) -> &str {
        match self {
            DataTypeKind::Integer => "INTEGER",
            DataTypeKind::Real => "REAL",
            DataTypeKind::Text => "TEXT",
            DataTypeKind::Blob => "BLOB",
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data_type.as_ref())?;

        if !self.optional {
            write!(f, " NOT NULL")?;
        }

        Ok(())
    }
}

impl From<DataTypeKind> for DataType {
    fn from(value: DataTypeKind) -> Self {
        Self {
            data_type: value,
            optional: false,
        }
    }
}

impl DataType {
    pub fn optional(self) -> Self {
        Self {
            optional: true,
            ..self
        }
    }
}

pub trait AsDataType {
    fn as_data_type() -> DataType;
}

impl AsDataType for i64 {
    fn as_data_type() -> DataType {
        DataTypeKind::Integer.into()
    }
}

impl AsDataType for String {
    fn as_data_type() -> DataType {
        DataTypeKind::Text.into()
    }
}

impl AsDataType for Key {
    fn as_data_type() -> DataType {
        DataTypeKind::Text.into()
    }
}

impl<T: AsDataType> AsDataType for Option<T> {
    fn as_data_type() -> DataType {
        T::as_data_type().optional()
    }
}
