use std::marker::PhantomData;

use sqlite::{State, Statement};

use crate::{filter::Filter, Link, Table};

pub struct SelectQuery<T: Table> {
  filter: Option<Filter>,
  phantom: PhantomData<T>,
}

impl<T: Table> SelectQuery<T> {
  pub fn new(filter: Filter) -> Self {
    Self {
      filter: Some(filter),
      phantom: PhantomData,
    }
  }

  pub fn id(id: i64) -> Self {
    Self::new(Filter::new(format!("{} = :filter", T::primary_name()), id))
  }

  pub fn link<U: Table, L: Table + Link<T> + Link<U>>(id: i64) -> Self {
    Self::new(Filter::new(
      format!(
        "{} IN (SELECT {} FROM {} WHERE {} = :filter)",
        T::primary_name(),
        <L as Link<T>>::get_name(),
        L::table_name(),
        <L as Link<U>>::get_name(),
      ),
      id,
    ))
  }

  pub fn all() -> Self {
    Self {
      filter: None,
      phantom: PhantomData,
    }
  }

  pub fn run(self, database: &crate::Database) -> sqlite::Result<SelectResult<T>> {
    let filter = if let Some(filter) = &self.filter {
      format!(" WHERE {}", filter.to_string())
    } else {
      "".to_string()
    };

    let q = format!("SELECT * FROM {}{}", T::table_name(), filter);

    let mut statement = database.prepare(q)?;

    if let Some(filter) = self.filter {
      filter.bind(&mut statement)?;
    }

    let mut data = vec![];

    while let State::Row = statement.next()? {
      data.push(T::read(&statement)?);
    }

    Ok(SelectResult {
      statement,
      phantom: PhantomData,
    })
  }
}

pub struct SelectResult<'a, T: Table> {
  statement: Statement<'a>,
  phantom: PhantomData<T>,
}

impl<'a, T: Table> SelectResult<'a, T> {
  pub fn read(&mut self) -> Option<sqlite::Result<T>> {
    match self.statement.next() {
      Ok(State::Row) => Some(T::read(&self.statement)),
      Ok(State::Done) => None,
      Err(err) => Some(Err(err)),
    }
  }
}
