mod error;
mod foo_data;
mod request;
mod tables;
mod web;
mod util;

pub use error::Error;

pub use request::Request;

use data::Database;
use tables::{
  category::Category, payment::Payment, payment::PaymentCategoryLink, payment::PaymentUserLink,
  user::User,
};
use web_server::HttpServer;

use crate::{
  foo_data::insert_foo,
  tables::{
    category::CategoryGroup,
    rule::{Rule, RuleCategoryLink}, tink_token::{TinkToken, TinkPayment},
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
  database.create::<TinkToken>().unwrap();
  database.create::<TinkPayment>().unwrap();

  insert_foo(&database);

  let server = HttpServer::new().not_found(Box::new(move |req, _| {
    web::handle(req, &database).unwrap_or_else(|e| e.into())
  }));
  println!("starting server on http://localhost:{}", port);
  server.launch(port);
}
