use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{BitAnd, BitOr},
};

use sqlite::{Statement, Value};

use crate::{Key, Table};

#[derive(Debug)]
pub enum FilterValue {
    Text(String),
    Int(i64),
    Null,
}

pub enum Filter<T: Table> {
    Eq(&'static str, FilterValue),
    Like(&'static str, FilterValue),
    In {
        own_column_name: &'static str,
        other_column_name: &'static str,
        other_table_name: &'static str,
        filter: Box<Self>,
    },
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Phantom(PhantomData<T>),
}

impl From<String> for FilterValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<i64> for FilterValue {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<Key> for FilterValue {
    fn from(value: Key) -> Self {
        Self::Text(value.into())
    }
}

impl<T: Into<FilterValue>> From<Option<T>> for FilterValue {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(inner) => inner.into(),
            None => FilterValue::Null,
        }
    }
}

impl Into<sqlite::Value> for FilterValue {
    fn into(self) -> sqlite::Value {
        match self {
            FilterValue::Text(text) => sqlite::Value::String(text),
            FilterValue::Int(int) => sqlite::Value::Integer(int),
            FilterValue::Null => sqlite::Value::Null,
        }
    }
}

impl<T: Table> BitAnd for Filter<T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::And(Box::new(self), Box::new(rhs))
    }
}

impl<T: Table> BitOr for Filter<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Or(Box::new(self), Box::new(rhs))
    }
}

impl<T: Table> Filter<T> {
    pub fn and(self, other: Self) -> Self {
        Self::And(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: Self) -> Self {
        Self::Or(Box::new(self), Box::new(other))
    }

    pub fn bind(self, statement: &mut Statement) -> sqlite::Result<()> {
        self.bind_counted(statement, &mut 0)
    }

    fn bind_counted(self, statement: &mut Statement, counter: &mut usize) -> sqlite::Result<()> {
        match self {
            Filter::Eq(_, value) | Filter::Like(_, value) => {
                *counter += 1;
                statement.bind::<(_, Value)>((counter.to_owned(), value.into()))
            }
            Filter::And(a, b) | Filter::Or(a, b) => {
                a.bind_counted(statement, counter)?;
                b.bind_counted(statement, counter)
            }
            Filter::In { filter, .. } => filter.bind_counted(statement, counter),
            Filter::Phantom(_) => Ok(()),
        }
    }
}

impl<T: Table> Display for Filter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::Eq(name, _) => {
                write!(f, "{} = ?", name)
            }
            Filter::Like(name, _) => {
                write!(f, "{} LIKE ?", name)
            }
            Filter::And(a, b) => {
                write!(f, "({}) AND ({})", a, b)
            }
            Filter::Or(a, b) => {
                write!(f, "({}) OR ({})", a, b)
            }
            Filter::In {
                own_column_name,
                other_column_name,
                other_table_name,
                filter,
            } => write!(
                f,
                "{} IN (SELECT {} FROM {} WHERE {})",
                own_column_name, other_column_name, other_table_name, filter
            ),
            Filter::Phantom(_) => Ok(()),
        }
    }
}
