use std::marker::PhantomData;

use crate::{Database, Table};

pub struct DeleteQuery<T: Table> {
  id: i64,
  phantom: PhantomData<T>,
}

impl<T: Table> DeleteQuery<T> {
  pub fn new(id: i64) -> Self {
    Self {
      id,
      phantom: PhantomData,
    }
  }

  pub fn run(&self, database: &Database) -> sqlite::Result<()> {
    let q = format!(
      "DELETE FROM {} WHERE {} = :id",
      T::table_name(),
      T::primary_name()
    );

    let mut statement = database.prepare(q)?;

    statement.bind((":id", self.id))?;

    statement.next()?;

    Ok(())
  }
}
