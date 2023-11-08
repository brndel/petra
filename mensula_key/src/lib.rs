use std::fmt::Display;

use serde::{Deserialize, Serialize};
use ulid::Ulid;

// use sqlite::ReadableWithIndex;
// use mensula::{table::DataTypeKind, AsDataType, DataType, FilterValue};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key {
    id: String,
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        &self.id
    }
}

impl Into<String> for Key {
    fn into(self) -> String {
        self.id
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.id)
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let id = String::deserialize(deserializer)?;
        Ok(Self { id })
    }
}

impl Key {
    pub fn new() -> Self {
        Self {
            id: Ulid::new().to_string(),
        }
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Self {
            id: value
        }
    }
}

#[cfg(feature = "sqlite")]
impl sqlite::ReadableWithIndex for Key {
    fn read<T: sqlite::ColumnIndex>(statement: &sqlite::Statement, index: T) -> sqlite::Result<Self> {
        let id = statement.read::<String, _>(index)?;
        Ok(Self { id })
    }
}

#[cfg(feature = "sqlite")]
impl sqlite::BindableWithIndex for Key {
    fn bind<T: sqlite::ParameterIndex>(self, statement: &mut sqlite::Statement, index: T) -> sqlite::Result<()> {
        let id: &str = &self.id;
        statement.bind((index, id))
    }
}