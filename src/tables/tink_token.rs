use data::{PrimKey, Table};

use super::{user::User, payment::Payment};

#[derive(Table)]
pub struct TinkToken {
  #[primary]
  #[foreign(User)]
  pub user_id: PrimKey,
  pub token: String,
  pub expires_timestamp: String,
}

#[derive(Table)]
pub struct TinkPayment {
  #[primary]
  #[foreign(Payment)]
  pub payment_id: PrimKey,
  pub tink_transaction_hash: String
}