use chrono::{DateTime, FixedOffset};
use data::{
  query::{Ordering, SelectQuery},
  Database, Table,
};
use serde::Serialize;

use crate::Error;

use super::{category::Category, user::User};

#[derive(Table, Debug)]
pub struct Payment {
  #[primary]
  pub id: i64,
  pub name: String,
  pub amount: i64,
  // The timestamp of the payment in the rfc3339 format (can include timezones)
  pub timestamp: String,
  #[foreign(User)]
  #[on_delete("restrict")]
  pub owner_id: i64,
}

#[derive(Serialize)]
pub struct PaymentResponse {
  id: i64,
  name: String,
  amount: i64,
  repay_amount: i64,
  timestamp: DateTime<FixedOffset>,
  owner_id: i64,
  users: Vec<i64>,
  categories: Vec<i64>,
}

impl Payment {
  pub fn get_payments(database: &Database, user_id: i64) -> Result<Vec<Payment>, Error> {
    Ok(
      SelectQuery::new()
        .filter(Self::owner_id().eq(user_id))
        .get_all(database),
    )
  }

  pub fn get_payments_date(
    database: &Database,
    user_id: i64,
    year: i64,
    month: i64,
  ) -> Result<Vec<Payment>, Error> {
    Ok(
      SelectQuery::new()
        .filter(
          Self::owner_id()
            .eq(user_id)
            .and(Self::timestamp().like(format!("{}-{:0>2}-%", year, month))),
        )
        .order_by(Self::timestamp(), Ordering::Descending)
        .get_all(database),
    )
  }

  pub fn into_response(self, database: &Database) -> PaymentResponse {
    let users = self.get_users(database);
    let repay_amount = self.get_repay_users(self.owner_id, &users);
    let categories = self.get_categories(database);
    PaymentResponse {
      id: self.id,
      name: self.name,
      amount: self.amount,
      repay_amount,
      timestamp: DateTime::parse_from_rfc3339(&self.timestamp).unwrap_or_default(),
      owner_id: self.owner_id,
      users,
      categories,
    }
  }

  fn get_users(&self, database: &Database) -> Vec<i64> {
    database.get_linked::<User, Payment, PaymentUserLink>(self.get_primary())
  }

  fn get_categories(&self, database: &Database) -> Vec<i64> {
    database.get_linked::<Category, Payment, PaymentCategoryLink>(self.get_primary())
  }

  pub fn get_repay(&self, user_id: i64, database: &Database) -> i64 {
    let users = self.get_users(database);

    self.get_repay_users(user_id, &users)
  }

  fn get_repay_users(&self, user_id: i64, users: &Vec<i64>) -> i64 {
    if users.is_empty() {
      return 0;
    }
    let amount_per_user = self.amount / users.len() as i64;
    let mut repay = if self.owner_id == user_id {
      -self.amount
    } else {
      0
    };
    if users.contains(&user_id) {
      repay += amount_per_user;
    }

    repay
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
