use std::marker::PhantomData;

use crate::{table::Insertable, Database, Table};

pub struct InsertQuery<I: Insertable<T>, T: Table> {
  data: I,
  phantom: PhantomData<T>,
}

impl<I: Insertable<T>, T: Table> InsertQuery<I, T> {
  pub fn new(data: I) -> Self {
    Self {
      data,
      phantom: PhantomData,
    }
  }

  pub fn run(self, database: &Database) -> sqlite::Result<i64> {
    let table_name = T::table_name();
    let primary_name = T::primary_name();

    let column_names = I::get_column_names().join(", ");
    let placeholder_names = I::get_placeholder_names().join(", ");

    let q = format!(
      "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
      table_name, column_names, placeholder_names, primary_name
    );

    let mut statement = database.prepare(q)?;

    self.data.bind(&mut statement)?;

    statement.next()?;

    let id = statement.read(primary_name)?;

    Ok(id)
  }
}
