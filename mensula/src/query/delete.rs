use std::marker::PhantomData;

use mensula_key::Key;

use crate::{Database, Table};

pub struct DeleteQuery<T: Table> {
  id: Key,
  phantom: PhantomData<T>,
}

impl<T: Table> DeleteQuery<T> {
  pub fn new(id: Key) -> Self {
    Self {
      id,
      phantom: PhantomData,
    }
  }

  pub fn run(&self, database: &Database) -> sqlite::Result<()> {
    let q = format!(
      "DELETE FROM {} WHERE {} = :id",
      T::table_name(),
      T::primary_column().name
    );

    let mut statement = database.prepare(q)?;

    statement.bind((":id", self.id.clone()))?;

    statement.next()?;

    Ok(())
  }
}
