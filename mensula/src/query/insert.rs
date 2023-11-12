use std::marker::PhantomData;

use crate::{table::Insertable, Database, Key, Table};

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

    pub fn run(self, database: &Database) -> sqlite::Result<Key> {
        let table_name = T::table_name();
        let primary_name = T::primary_column().name;

        let column_names = I::get_column_names().join(", ");
        let placeholder_names = I::get_placeholder_names().join(", ");

        let update_columns = I::get_column_names()
            .iter()
            .map(|name| format!("{}=excluded.{}", name, name))
            .collect::<Vec<_>>()
            .join(", ");

        let q = format!(
            "INSERT INTO {} ({}) VALUES ({})
            ON CONFLICT ({}) DO UPDATE SET {}
            RETURNING {}",
            // Insert
            table_name,
            column_names,
            // Values
            placeholder_names,
            // Conflict
            primary_name,
            update_columns,
            // Return
            primary_name
        );

        let mut statement = database.prepare(q)?;

        self.data.bind(&mut statement)?;

        statement.next()?;

        let id: String = statement.read(primary_name)?;

        Ok(Key::from(id))
    }
}
