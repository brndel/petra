use chrono::DateTime;
use data::Database;
use serde::{Deserialize, Serialize};

use crate::{
  tables::payment::{Payment, PaymentCategoryLinkInsert, PaymentInsert, PaymentUserLinkInsert},
  Error, Request,
};

use super::serialize;

pub fn get_payments(request: &Request) -> Result<String, Error> {
  let date = request
    .params
    .get("month")
    .ok_or(Error::BadRequest("Missing 'month' parameter".to_string()))?;

  let parts: Vec<_> = date.split('-').collect();

  let year = parts
    .get(0)
    .ok_or(Error::BadRequest("invalid format for 'month'".to_string()))?;
  let month = parts
    .get(1)
    .ok_or(Error::BadRequest("invalid format for 'month'".to_string()))?;

  let year = year
    .parse()
    .map_err(|_| Error::BadRequest("could not parse year".to_string()))?;
  let month = month
    .parse()
    .map_err(|_| Error::BadRequest("could not parse month".to_string()))?;

  let payments = Payment::get_payments_date(request.database, request.user_id, year, month)?;
  let payments: Vec<_> = payments
    .into_iter()
    .map(|e| e.into_response(request.database))
    .collect();

  serialize(&payments)
}

#[derive(Deserialize)]
struct PaymentPostData {
  name: String,
  amount: i64,
  timestamp: String,
  users: Vec<i64>,
  categories: Vec<i64>,
}

#[derive(Serialize)]
enum PaymentError {
  Format(PaymentFormatError),
  Insert(Vec<PaymentInsertError>),
}

#[derive(Serialize)]
enum PaymentFormatError {
  NoUsers,
  InvalidTimestamp,
}

#[derive(Serialize)]
enum PaymentInsertError {
  InvalidPayment(String, i64),
  InvalidUser(i64),
  InvalidCategory(i64),
}

pub fn post_payments(request: &Request) -> Result<String, Error> {
  let data: Vec<PaymentPostData> = serde_json::from_str(&request.body)
    .map_err(|_| Error::BadRequest("could not deserialize request".to_string()))?;

  let mut errors = Vec::new();

  for payment_data in data {
    if let Err(e) = create_payment(payment_data, request) {
      errors.push(e);
    }
  }

  if errors.is_empty() {
    Ok("null".to_string())
  } else {
    Err(Error::BadRequest(serialize(&errors)?))
  }
}

fn create_payment(data: PaymentPostData, request: &Request) -> Result<(), PaymentError> {
  if let Err(e) = validate_payment(&data) {
    return Err(PaymentError::Format(e));
  }

  insert_payment(data, request.database, request.user_id).map_err(|e| PaymentError::Insert(e))
}

fn validate_payment(data: &PaymentPostData) -> Result<(), PaymentFormatError> {
  if data.users.is_empty() {
    return Err(PaymentFormatError::NoUsers);
  }

  if let Err(_) = DateTime::parse_from_rfc3339(&data.timestamp) {
    return Err(PaymentFormatError::InvalidTimestamp);
  }

  Ok(())
}

fn insert_payment(
  data: PaymentPostData,
  database: &Database,
  owner_id: i64,
) -> Result<(), Vec<PaymentInsertError>> {
  let name = data.name;
  let payment_id = database
    .insert(PaymentInsert {
      name: name.clone(),
      amount: data.amount,
      timestamp: data.timestamp,
      owner_id,
    })
    .ok_or_else(|| vec![PaymentInsertError::InvalidPayment(name, owner_id)])?;

  let mut errors = Vec::new();

  for user_id in data.users {
    if let None = database.insert(PaymentUserLinkInsert {
      payment_id,
      user_id,
    }) {
      errors.push(PaymentInsertError::InvalidUser(user_id));
    }
  }

  for category_id in data.categories {
    if let None = database.insert(PaymentCategoryLinkInsert {
      payment_id,
      category_id,
    }) {
      errors.push(PaymentInsertError::InvalidCategory(category_id))
    }
  }

  if errors.is_empty() {
    Ok(())
  } else {
    Err(errors)
  }
}
