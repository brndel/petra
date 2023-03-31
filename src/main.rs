mod tables;
mod web;

extern crate data;
#[macro_use]
extern crate data_derive;

use data::Database;
use tables::user::User;

use crate::tables::{
  category::Category, link_payment_category::LinkPaymentCategory, payment::Payment,
};

fn main() {
  let database = Database::open("data.sqlite").unwrap();

  database.create::<User>().unwrap();
  database.create::<Payment>().unwrap();
  database.create::<Category>().unwrap();
  database.create::<LinkPaymentCategory>().unwrap();
}
