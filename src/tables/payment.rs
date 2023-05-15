use chrono::{DateTime, FixedOffset};
use data::{
  query::{Ordering, SelectQuery},
  Database, Table, Filter,
};
use serde::Serialize;

use crate::{Error, util::ExactMonth, Request};

use super::{category::Category, user::User};

#[derive(Table, Debug)]
pub struct Payment {
  #[primary]
  pub id: i64,
  pub name: String,
  pub amount: i64,
  pub original_amount: i64,
  // The timestamp of the payment in the rfc3339 format (can include timezones)
  pub timestamp: String,
  pub original_timestamp: String,
  #[foreign(User)]
  #[on_delete("restrict")]
  pub owner_id: i64,
}

#[derive(Serialize)]
pub struct PaymentResponse {
  id: i64,
  name: String,
  real_amount: i64,
  user_amount: i64,
  repay_amount: i64,
  timestamp: DateTime<FixedOffset>,
  owner_id: i64,
  users: Vec<i64>,
  categories: Vec<i64>,
}

impl Payment {
  fn user_filter(user_id: i64) -> Filter<Payment> {
    Self::owner_id().eq(user_id).or(Self::primary_column().link::<PaymentUserLink, User>(user_id))
  }

  pub fn get_payments(database: &Database, user_id: i64) -> Result<Vec<Payment>, Error> {
    SelectQuery::new()
      .filter(Self::user_filter(user_id))
      .get_all(database)
      .ok_or(Error::Database)
  }

  pub fn get_payments_by_date(
    database: &Database,
    user_id: i64,
    month: ExactMonth
  ) -> Result<Vec<Payment>, Error> {
    SelectQuery::new()
      .filter(
        Self::timestamp().like(format!("{}-%", month))
        .and(
          Self::user_filter(user_id)
        ),
      )
      .order_by(Self::timestamp(), Ordering::Descending)
      .get_all(database)
      .ok_or(Error::Database)
  }

  pub fn into_response(self, request: &Request) -> PaymentResponse {
    let users = self.get_users(request.database);
    let (user_amount, repay_amount) = self.get_repay_users(request.user_id, &users);
    let categories = self.get_categories(request.database);
    PaymentResponse {
      id: self.id,
      name: self.name,
      real_amount: self.amount,
      user_amount,
      repay_amount,
      timestamp: DateTime::parse_from_rfc3339(&self.timestamp).unwrap_or_default(),
      owner_id: self.owner_id,
      users,
      categories,
    }
  }

  fn get_users(&self, database: &Database) -> Vec<i64> {
    database
      .get_linked::<User, Payment, PaymentUserLink>(self.get_primary())
      .unwrap_or_default()
  }

  fn get_categories(&self, database: &Database) -> Vec<i64> {
    database
      .get_linked::<Category, Payment, PaymentCategoryLink>(self.get_primary())
      .unwrap_or_default()
  }

  pub fn get_repay(&self, user_id: i64, database: &Database) -> (i64, i64) {
    let users = self.get_users(database);

    self.get_repay_users(user_id, &users)
  }

  fn get_repay_users(&self, user_id: i64, users: &Vec<i64>) -> (i64, i64) {
    if users.is_empty() {
      return (0, 0);
    }

    let amount_per_user = self.amount / users.len() as i64;
    let is_owner = self.owner_id == user_id;
    let is_user = users.contains(&user_id);

    let mut amount = 0;
    if is_owner {
      amount = self.amount;
    }

    let mut repay = -amount;
    if is_user {
      repay += amount_per_user;
    }

    (amount, repay)
  }
}

// Links

#[derive(Table)]
pub struct PaymentUserLink {
  #[primary]
  pub id: i64,
  #[foreign_link(Payment)]
  pub payment_id: i64,
  #[foreign_link(User)]
  pub user_id: i64,
}

#[derive(Table)]
pub struct PaymentCategoryLink {
  #[primary]
  pub id: i64,
  #[foreign_link(Payment)]
  pub payment_id: i64,
  #[foreign_link(Category)]
  pub category_id: i64,
}
