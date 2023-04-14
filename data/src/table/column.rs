use std::{fmt::Display, marker::PhantomData};

use crate::{filter::FilterValue, DataType, Filter, Link, Modifier, Table};

use super::PrimKey;

pub struct Column<T: Table> {
  pub name: &'static str,
  data_type: DataType,
  modifier: Modifier,
  phantom: PhantomData<T>,
}

impl<T: Table> Column<T> {
  pub const fn new(name: &'static str, data_type: DataType, modifier: Modifier) -> Self {
    Self {
      name,
      data_type,
      modifier,
      phantom: PhantomData,
    }
  }

  pub fn eq<V: Into<FilterValue>>(&self, value: V) -> Filter<T> {
    Filter::Eq(self.name, value.into(), PhantomData)
  }

  pub fn like(&self, value: String) -> Filter<T> {
    Filter::Like(self.name, value.into())
  }

  pub fn link<L: Table + Link<T> + Link<U>, U: Table>(&self, value: PrimKey) -> Filter<T> {
    Filter::In {
      own_column_name: T::primary_column().name,
      other_column_name: <L as Link<T>>::link_name(),
      other_table_name: L::table_name(),
      filter: Box::new(Filter::Eq(<L as Link<U>>::link_name(), value.into(), PhantomData)),
    }
  }
}

impl<T: Table> Display for Column<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}{}", self.name, self.data_type, self.modifier,)
  }
}
