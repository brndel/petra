use chrono::{Local, DateTime};

use crate::{Request, tables::tink_token::TinkToken};

pub mod payment;
pub mod token;
pub mod tink_secret;
mod transaction;

fn get_token(request: &Request) -> Option<TinkToken> {
  let token = request.database.get::<TinkToken>(request.user_id)?;


  let now = Local::now();
  let expires = DateTime::parse_from_rfc3339(&token.expires_timestamp);
  if let Ok(expires) = expires {
    if now < expires {
      return Some(token);
    }
  }

  if let Err(e) = request.database.delete::<TinkToken>(request.user_id) {
    println!("[tink] error {}", e);
  }

  return None;
}
