use std::{fmt::Display, marker::PhantomData};

use sqlite::{Statement, Value};

use crate::Table;

pub enum FilterValue {
  Text(String),
  Int(i64),
  Null,
}

pub enum Filter<T: Table> {
  Eq(&'static str, FilterValue, PhantomData<T>),
  Like(&'static str, FilterValue),
  In {
    own_column_name: &'static str,
    other_column_name: &'static str,
    other_table_name: &'static str,
    filter: Box<Self>,
  },
  And(Box<Self>, Box<Self>),
  Or(Box<Self>, Box<Self>),
}

impl Into<FilterValue> for String {
  fn into(self) -> FilterValue {
    FilterValue::Text(self)
  }
}

impl Into<FilterValue> for i64 {
  fn into(self) -> FilterValue {
    FilterValue::Int(self)
  }
}

impl<T: Into<FilterValue>> Into<FilterValue> for Option<T> {
  fn into(self) -> FilterValue {
    match self {
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
    *counter += 1;
    match self {
      Filter::Eq(_, value, _) | Filter::Like(_, value) => {
        statement.bind::<(_, Value)>((counter.to_owned(), value.into()))
      }
      Filter::And(a, b) | Filter::Or(a, b) => {
        a.bind_counted(statement, counter)?;
        b.bind_counted(statement, counter)
      }
      Filter::In { filter, .. } => filter.bind_counted(statement, counter),
    }
  }
}

impl<T: Table> Display for Filter<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Filter::Eq(name, _, _) => {
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
    }
  }
}
