use data::Table;

use super::payment::Payment;
use super::category::Category;

#[derive(Table)]
pub struct LinkPaymentCategory {
  #[primary]
  pub id: i64,
  #[foreign_link(Payment)]
  pub payment_id: i64,
  #[foreign_link(Category)]
  pub category_id: i64,
}