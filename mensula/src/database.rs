use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};

use sqlite::{Connection, Statement};

use crate::meta::{Difference, Meta};
use crate::query::{CreateTableQuery, DeleteQuery, InsertQuery, SelectQuery};
use crate::table::{Insertable, Readable};
use crate::{Key, Link, Table};

pub struct Database {
    connection: Connection,
    meta: Meta,
    meta_path: PathBuf,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, sqlite::Error> {
        let path = path.as_ref().to_owned();

        let connection = sqlite::open(&path)?;

        let mut meta_path = path;
        meta_path.set_extension("meta.toml");

        let meta = match fs::read_to_string(&meta_path) {
            Ok(s) => toml::from_str(&s).expect(&format!(
                "could not read DB meta file at '{}'",
                meta_path.display()
            )),
            Err(_) => Meta::default(),
        };

        connection.execute("PRAGMA foreign_keys = ON")?;

        Ok(Self {
            connection,
            meta,
            meta_path,
        })
    }

    pub fn register<T: Table>(&mut self) -> sqlite::Result<()> {
        let difference = match self.meta.get_difference::<T>() {
            Some(difference) => difference,
            None => return Ok(()),
        };

        match difference {
            Difference::NewTable => self.create_table::<T>(),
            Difference::Columns(difference) => {
                println!(
                    "found difference in table '{}':\n{:?}",
                    T::table_name(),
                    difference
                );
                todo!()
            }
        }
    }

    fn save_meta(&self) {
        self.meta.save(&self.meta_path);
    }

    pub(crate) fn execute<S: AsRef<str>>(&self, query: S) -> sqlite::Result<()> {
        let query = query.as_ref();
        let result = self.connection.execute(query);
        match result {
            Ok(value) => Ok(value),
            Err(err) => {
                println!("failed to execute '{}': {}", query, err);
                Err(err)
            }
        }
    }

    pub(crate) fn prepare<S: AsRef<str>>(&self, query: S) -> sqlite::Result<Statement> {
        self.connection.prepare(query)
    }
}

impl Database {
    fn create_table<T: Table>(&mut self) -> sqlite::Result<()> {
        CreateTableQuery::<T>::new().run(&self)?;

        self.meta.update_table::<T>();
        self.save_meta();

        Ok(())
    }

    pub fn insert<I: Insertable<impl Table>>(&self, data: I) -> Option<Key> {
        match InsertQuery::new(data).run(&self) {
            Ok(key) => Some(key),
            Err(err) => {
                println!("{:?}", err);
                None
            }
        }
    }

    pub fn get_all<T: Table + Readable<T>>(&self) -> Option<Vec<T>> {
        SelectQuery::<T>::new().get_all(self)
    }

    pub fn get<T: Table + Readable<T>>(&self, key: Key) -> Option<T> {
        if !self.meta.has_table::<T>() {
            println!("Table '{}' not registered", T::table_name());
            return None;
        }
        SelectQuery::new()
            .filter(T::primary_column().eq(key))
            .get_first(self)
    }

    pub fn get_linked<T: Table + Readable<R>, R, U: Table, L: Link<T> + Link<U> + Table>(
        &self,
        key: Key,
    ) -> Option<Vec<R>> {
        SelectQuery::new()
            .filter(T::primary_column().link::<L, U>(key))
            .get_all(self)
    }

    pub fn delete<T: Table>(&self, id: Key) -> sqlite::Result<()> {
        DeleteQuery::<T>::new(id).run(self)
    }
}

impl Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database")
            .field("connection", &"[...]".to_string())
            .field("meta", &self.meta)
            .field("meta_path", &self.meta_path)
            .finish()
    }
}
