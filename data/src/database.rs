use sqlite::{Connection, Statement};

use crate::query::{CreateTableQuery, InsertQuery, SelectQuery, DeleteQuery};
use crate::table::Insertable;
use crate::Table;

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

  pub fn insert<I: Insertable<impl Table>>(&self, data: I) -> sqlite::Result<i64> {
    InsertQuery::new(data).run(&self)
  }

  pub fn get<T: Table>(&self, query: SelectQuery<T>) -> sqlite::Result<T> {
    let mut result = query.run(&self)?;
    if let Some(result) = result.read() {
      result
    } else {
      Err(sqlite::Error {
        code: None,
        message: Some(format!(
          "Nothing found in {}",
          T::table_name()
        )),
      })
    }
  }

  pub fn get_vec<T: Table>(&self, query: SelectQuery<T>) -> sqlite::Result<Vec<T>> {
    let mut result = query.run(&self)?;

    let mut data = Vec::new();

    while let Some(result) = result.read() {
      let result = result?;
      data.push(result);
    }

    Ok(data)
  }

  pub fn get_all<T: Table>(&self) -> sqlite::Result<Vec<T>> {
    self.get_vec(SelectQuery::all())
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
