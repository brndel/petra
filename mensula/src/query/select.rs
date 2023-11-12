use std::{fmt::Display, marker::PhantomData};

use mensula_key::Key;
use sqlite::{State, Statement};

use crate::{filter::Filter, table::Readable, Column, Database, Table, Link};

pub enum Ordering {
  Ascending,
  Descending,
}

impl Display for Ordering {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Ordering::Ascending => "ASC",
        Ordering::Descending => "DESC",
      }
    )
  }
}

pub struct SelectQuery<T: Table> {
  filter: Option<Filter<T>>,
  ordering: Option<(&'static str, Ordering)>,
  phantom: PhantomData<T>,
}

impl<T: Table> SelectQuery<T> {
  pub fn new() -> Self {
    Self {
      filter: None,
      ordering: None,
      phantom: PhantomData,
    }
  }

  pub fn link<U: Table, L: Link<T> + Link<U> + Table>(key: Key) -> Self {
    SelectQuery::new().filter(T::primary_column().link::<L, U>(key))
  }

  // Builders

  pub fn filter(mut self, filter: Filter<T>) -> Self {
    self.filter = Some(filter);
    self
  }

  pub fn order_by(mut self, column: Column<T>, ordering: Ordering) -> Self {
    self.ordering = Some((column.name, ordering));
    self
  }

  // Runners

  pub fn get_query<R>(&self) -> String
  where
    T: Readable<R>,
  {
    let column_names = if let Some(cols) = T::get_column_names() {
      cols.join(", ")
    } else {
      "*".to_string()
    };

    let mut query = format!("SELECT {} FROM {}", column_names, T::table_name());

    if let Some(filter) = &self.filter {
      query += format!(" WHERE {}", filter).as_str();
    }

    if let Some((order_by, ordering)) = &self.ordering {
      query += format!(" ORDER BY {} {}", order_by, ordering).as_str();
    }

    query
  }

  fn run<R>(self, database: &Database) -> sqlite::Result<Statement>
  where
    T: Readable<R>,
  {
    let q = self.get_query();

    let mut statement = database.prepare(q)?;

    if let Some(filter) = self.filter {
      filter.bind(&mut statement)?;
    }

    Ok(statement)
  }

  pub fn get_all<R>(self, database: &Database) -> Option<Vec<R>>
  where
    T: Readable<R>,
  {
    let mut data = Vec::new();

    let mut statement = self.run(database).ok()?;

    while let State::Row = statement.next().ok()? {
      data.push(T::read(&statement)?);
    }

    Some(data)
  }

  pub fn get_first<R>(self, database: &Database) -> Option<R>
  where
    T: Readable<R>,
  {
    let mut statement = self.run(database).ok()?;
    if let State::Row = statement.next().ok()? {
      T::read(&statement)
    } else {
      None
    }
  }
}

// impl<T: Table + Readable<R>, R> SelectQuery<T> {

// }
