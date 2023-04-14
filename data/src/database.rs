use std::fmt::Debug;

use sqlite::{Connection, Statement};

use crate::query::{CreateTableQuery, DeleteQuery, InsertQuery, SelectQuery};
use crate::table::{Insertable, Readable};
use crate::{Table, PrimKey, Link};

pub struct Database {
  connection: Connection,
}

impl Database {
  pub fn open(path: &str) -> Result<Self, sqlite::Error> {
    let connection = sqlite::open(path)?;

    connection.execute("PRAGMA foreign_keys = ON")?;

    Ok(Self { connection })
  }

  pub fn create<T: Table>(&self) -> sqlite::Result<()> {
    CreateTableQuery::<T>::new().run(&self)
  }

  pub fn insert<I: Insertable<impl Table>>(&self, data: I) -> Option<i64> {
    InsertQuery::new(data).run(&self).ok()
  }

  pub fn get_all<T: Table + Readable<T>>(&self) -> Vec<T> {
    SelectQuery::<T>::new().get_all(self)
  }

  pub fn get<T: Table + Readable<T>>(&self, primary: PrimKey) -> Option<T> {
    SelectQuery::new().filter(T::primary_column().eq(primary)).get_first(self)
  }

  pub fn get_linked<T: Table + Readable<PrimKey>, U: Table, L: Link<T> + Link<U> + Table>(&self, primary: PrimKey) -> Vec<PrimKey> {
    SelectQuery::new().filter(T::primary_column().link::<L, U>(primary)).get_all(self)
  }

  pub fn delete<T: Table>(&self, id: i64) -> sqlite::Result<()> {
    DeleteQuery::<T>::new(id).run(self)
  }

  pub(crate) fn execute<S: AsRef<str>>(&self, query: S) -> sqlite::Result<()> {
    // println!("{}", query.as_ref());
    self.connection.execute(query)
  }

  pub(crate) fn prepare<S: AsRef<str>>(&self, query: S) -> sqlite::Result<Statement> {
    // println!("{}", query.as_ref());
    self.connection.prepare(query)
  }
}


impl Debug for Database {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Database")
      .field("connection", &"[...]".to_string())
      .finish()
  }
}