mod error;
mod foo_data;
mod request;
mod tables;
mod web;

pub use error::Error;

pub use request::Request;

extern crate data;
// #[macro_use]
// extern crate data_derive;

use data::{query::SelectQuery, Database, PrimKey};
use tables::{
  category::Category, payment::Payment, payment::PaymentCategoryLink, payment::PaymentUserLink,
  user::User,
};
use web_server::HttpServer;

use crate::{
  foo_data::{insert_foo, ADMIN_USER_ID},
  tables::{
    category::CategoryGroup,
    rule::{Rule, RuleCategoryLink},
  },
};

fn main() {
  let port = 8187;

  let database = Database::open("data.sqlite").unwrap();

  database.create::<User>().unwrap();
  database.create::<CategoryGroup>().unwrap();
  database.create::<Category>().unwrap();
  database.create::<Payment>().unwrap();
  database.create::<PaymentCategoryLink>().unwrap();
  database.create::<PaymentUserLink>().unwrap();
  database.create::<Rule>().unwrap();
  database.create::<RuleCategoryLink>().unwrap();

  if SelectQuery::new().filter(User::id().eq(ADMIN_USER_ID)).get_first::<PrimKey>(&database).is_none()
  {
    insert_foo(&database);
  }

  let server = HttpServer::new().not_found(Box::new(move |req, _| {
    web::handle(req, &database).unwrap_or_else(|e| e.into())
  }));
  println!("starting server on http://localhost:{}", port);
  server.launch(port);
}
