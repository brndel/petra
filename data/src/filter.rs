// pub struct ColumnFilter<T: Into<Value>> {
//   column_name: &'static str,
//   phantom: PhantomData<T>,
// }

// impl<T: Into<Value>> ColumnFilter<T> {
//   fn eq(&self, value: T) -> Filter<T> {
//     Filter {
//       column_name: self.column_name,
//       method: FilterMethod::Equal,
//       value,
//     }
//   }
// }

use sqlite::{Value, Statement};

pub struct Filter {
  // column_name: &'static str,
  // method: FilterMethod,
  // value: T,
  filter: String,
  value: Value
}

impl Filter {
  pub fn new<T: Into<Value>>(filter: String, value: T) -> Self {
    Self { filter, value: value.into() }
  }

  pub fn bind(self, statement: &mut Statement) -> sqlite::Result<()> {
    statement.bind((":filter", self.value))
  }

  pub fn to_string(&self) -> String {
    self.filter.clone()
  }
}

// impl<T: Into<Value>> Filter<T> {
//   pub fn to_string(&self) -> String{
//     format!("{} {} {}", self.column_name, self.method.as_ref(), ":filter")
//   }

//   pub fn bind(self, statement: &mut Statement) -> sqlite::Result<()>{
//     statement.bind((":filter", self.value.into()))
//   }
// }

// enum FilterMethod {
//   Equal,
//   NotEqual,
//   Less,
//   Greater,
//   LessEqual,
//   GreaterEqual,
//   In,
// }

// impl AsRef<str> for FilterMethod {
//   fn as_ref(&self) -> &str {
//     match self {
//       FilterMethod::Equal => "=",
//       FilterMethod::NotEqual => "!=",
//       FilterMethod::Less => "<",
//       FilterMethod::Greater => ">",
//       FilterMethod::LessEqual => "<=",
//       FilterMethod::GreaterEqual => ">=",
//       FilterMethod::In => "IN",
//     }
//   }
// }

// struct FooFilter {
//   id: ColumnFilter<i64>,
//   name: ColumnFilter<String>,
//   display_name: ColumnFilter<String>,
// }

// fn foo() {
//   let f = FooFilter {
//     id: todo!(),
//     name: todo!(),
//     display_name: todo!(),
//   };

//   f.display_name.eq(String::from("abc"));
//   f.id.eq(123);
// }
