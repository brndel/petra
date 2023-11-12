use std::marker::PhantomData;

use crate::{Database, Table};

pub struct CreateTableQuery<T: Table> {
    phantom: PhantomData<T>,
}

impl<T: Table> CreateTableQuery<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn run(self, database: &Database) -> sqlite::Result<()> {
        let columns: Vec<String> = T::get_columns().iter().map(|c| c.to_string()).collect();
        let columns = columns.join(", ");

        let q = format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            T::table_name(),
            columns
        );

        database.execute(q)
    }
}
