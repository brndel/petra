use std::{collections::HashMap, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{table::DataTypeKind, Table};

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    version: String,
    tables: HashMap<String, MetaTable>,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            version: "0.1.0".into(),
            tables: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MetaTable {
    primary: String,
    columns: HashMap<String, MetaColumn>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MetaColumn {
    data_type: DataTypeKind,
    optional: bool,
    unique: bool,
}

pub enum Difference {
    NewTable,
    Columns(HashMap<String, ColumnDifference>),
}

#[derive(Debug)]
pub struct ColumnDifference {
    pub before: Option<MetaColumn>,
    pub after: Option<MetaColumn>,
}

impl Meta {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> bool {
        let content = match toml::to_string(self) {
            Ok(content) => content,
            Err(_) => return false,
        };
        fs::write(path, content).is_ok()
    }

    pub fn get_difference<T: Table>(&self) -> Option<Difference> {
        let name = T::table_name();

        if let Some(table) = self.get_table(name) {
            let difference = table.compare(&Self::get_meta_table::<T>());

            if difference.is_empty() {
                None
            } else {
                Some(Difference::Columns(difference))
            }
        } else {
            Some(Difference::NewTable)
        }
    }

    pub fn has_table<T: Table>(&self) -> bool {
        self.tables.contains_key(T::table_name())
    }

    fn get_table(&self, name: &str) -> Option<&MetaTable> {
        self.tables.get(name)
    }

    fn get_meta_table<T: Table>() -> MetaTable {
        let column_iter = T::get_columns().into_iter().map(|col| {
            (
                col.name.to_owned(),
                MetaColumn {
                    data_type: col.data_type.data_type,
                    optional: col.data_type.optional,
                    unique: col.modifier.unique,
                },
            )
        });

        let columns = HashMap::from_iter(column_iter);

        MetaTable {
            primary: T::primary_column().name.to_owned(),
            columns,
        }
    }

    pub fn update_table<T: Table>(&mut self) {
        self.tables
            .insert(T::table_name().to_owned(), Self::get_meta_table::<T>());
    }
}

impl MetaTable {
    pub fn compare(&self, other: &Self) -> HashMap<String, ColumnDifference> {
        let mut map = HashMap::new();

        for (name, column) in &self.columns {
            if let Some(other_column) = other.columns.get(name) {
                if column != other_column {
                    map.insert(
                        name.to_owned(),
                        ColumnDifference {
                            before: Some(column.to_owned()),
                            after: Some(other_column.to_owned()),
                        },
                    );
                }
            } else {
                map.insert(
                    name.to_owned(),
                    ColumnDifference {
                        before: Some(column.to_owned()),
                        after: None,
                    },
                );
            }
        }

        for (name, other_column) in &other.columns {
            if !self.columns.contains_key(name) {
                map.insert(
                    name.to_owned(),
                    ColumnDifference {
                        before: None,
                        after: Some(other_column.to_owned()),
                    },
                );
            }
        }

        map
    }
}
