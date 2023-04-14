use std::collections::HashMap;

use chrono::{DateTime, Datelike};
use serde::Serialize;
use serde_json::json;

use crate::{tables::payment::Payment, Error, Request};

use super::serialize;

#[derive(Serialize)]
struct MonthResult {
  month: String,
  positive: i64,
  negative: i64,
  repay: i64,
}

impl MonthResult {
  fn new(month: &String) -> Self {
    Self {
      month: month.clone(),
      positive: 0,
      negative: 0,
      repay: 0,
    }
  }
}

pub fn get_month_index(request: &Request) -> Result<String, Error> {
  let payments = Payment::get_payments(request.database, request.user_id)?;

  let mut sum_month = MonthResult::new(&"*".to_string());

  let mut months = HashMap::<String, MonthResult>::new();

  for payment in payments {
    let date = DateTime::parse_from_rfc3339(&payment.timestamp)
      .unwrap_or_default()
      .date_naive();

    let month = format!("{}-{:0>2}", date.year(), date.month());

    let month_result = if let Some(result) = months.get_mut(&month) {
      result
    } else {
      let result = MonthResult::new(&month);
      months.insert(month.clone(), result);
      months.get_mut(&month).unwrap()
    };

    if payment.amount > 0 {
      month_result.positive += payment.amount;
      sum_month.positive += payment.amount;
    } else if payment.amount < 0 {
      month_result.negative += payment.amount;
      sum_month.negative += payment.amount;
    }

    let repay = payment.get_repay(request.user_id, request.database);

    month_result.repay += repay;
    sum_month.repay += repay;
  }

  let mut months: Vec<MonthResult> = months.into_values().collect();
  months.sort_by(|a, b| b.month.cmp(&a.month));

  let result = json!({"sum": sum_month, "months": months});

  serialize(&result)
}
