use sqlite::Statement;

use crate::Column;

use super::PrimKey;

pub trait Table
where
  Self: Sized,
{
  fn table_name() -> &'static str;

  fn primary_column() -> Column<Self>;
  fn get_columns() -> Vec<Column<Self>>;

  fn get_primary(&self) -> PrimKey;
}

pub trait Readable<R> {
  fn get_column_names() -> Option<&'static [&'static str]>;
  fn read(statement: &sqlite::Statement) -> Option<R>;
}

pub trait Insertable<T: Table> {
  fn get_column_names() -> &'static [&'static str];
  fn get_placeholder_names() -> &'static [&'static str];

  fn bind(self, statement: &mut Statement) -> sqlite::Result<()>;
}

pub trait Updatable<T: Table> {
  fn get_column_names() -> &'static [&'static str];
  fn get_placeholder_names() -> &'static [&'static str];

  fn bind(self, statement: &mut Statement) -> sqlite::Result<()>;
}

pub trait Link<T: Table> {
  fn link_name() -> &'static str;
}

impl<T: Table + Updatable<T>> Insertable<T> for T {
    fn get_column_names() -> &'static [&'static str] {
        T::get_column_names()
    }

    fn get_placeholder_names() -> &'static [&'static str] {
        T::get_placeholder_names()
    }

    fn bind(self, statement: &mut Statement) -> sqlite::Result<()> {
        T::bind(self, statement)
    }
}

// FOO DATA

// struct Foo {
//   id: i64,
//   name: String,
// }

// impl Foo {
//   fn id() -> Column<Self> {
//     Column::new("id", todo!(), todo!())
//   }

//   fn name() -> Column<Self> {
//     Column::new("name", todo!(), todo!())
//   }
// }

// impl Table for Foo {
//   fn table_name() -> &'static str {
//     "Foo"
//   }

//   fn primary_column() -> Column<Self> {
//     Column::new("id", todo!(), todo!())
//   }

//   fn get_columns() -> Vec<Column<Self>> {
//     vec![
//       Column::new("id", todo!(), todo!()),
//       Column::new("name", todo!(), todo!()),
//     ]
//   }

//   fn get_primary(&self) -> PrimKey {
//     self.id
//   }
// }

// impl Readable<Foo> for Foo {
//   fn get_column_names() -> Option<&'static [&'static str]> {
//     None
//   }

//   fn read(statement: &sqlite::Statement) -> Option<Foo> {
//     Some(Self {
//       id: statement.read("id").ok()?,
//       name: statement.read("name").ok()?,
//     })
//   }
// }

// impl Readable<PrimKey> for Foo {
//   fn get_column_names() -> Option<&'static [&'static str]> {
//     Some(&["id"])
//   }

//   fn read(statement: &sqlite::Statement) -> Option<PrimKey> {
//     Some(statement.read("id").ok()?)
//   }
// }

// fn read_foo(database: &Database) {
//   let foo_id: i64 = SelectQuery::new()
//     .filter(Foo::id().eq(5))
//     .get_first(database)
//     .unwrap();
// }
