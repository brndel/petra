use serde::Deserialize;
use serde_json::json;

use crate::{
  tables::tink_token::TinkToken,
  util::ExactMonth,
  web::api::{serialize, tink::transaction::ResponsePayment},
  Error, Request,
};

use super::{get_token, transaction::TinkTransaction};

#[derive(Deserialize)]
struct TinkPaymentResponse {
  #[serde(rename = "nextPageToken")]
  next_page_token: String,
  transactions: Vec<TinkTransaction>,
}

pub fn get_tink_payments(request: &Request) -> Result<String, Error> {
  let month = request
    .params
    .get("month")
    .ok_or(Error::BadRequest("Missing 'month' parameter".to_string()))?;

  let month: ExactMonth = month
    .parse()
    .map_err(|_| Error::BadRequest("Invalid 'month' format".to_string()))?;

  let transactions = get_payments(request, &month).ok_or(Error::Internal)?;

  let payments: Vec<ResponsePayment> = transactions.into_iter().filter_map(|tr| tr.try_into().ok()).collect();

  let mut booked_payments = Vec::new();
  let mut pending_payments = Vec::new();
  let mut listed_payments = Vec::new();

  println!("ALL PAYMENTS: {:?}", payments.len());

  for p in payments {
    if p.is_booked() {
      if p.is_listed(request.database) {
        listed_payments.push(p);
      } else {
        booked_payments.push(p);
      }
    } else {
      pending_payments.push(p);
    }
  }

  let result = json!({
    "new": booked_payments,
    "pending": pending_payments.len(),
    "listed": listed_payments.len(),
  });

  serialize(&result)
}

fn get_payments(request: &Request, month: &ExactMonth) -> Option<Vec<TinkTransaction>> {
  println!("TINK REQUEST");
  let token = get_token(request)?;
  println!("Valid Token");

  let date_first = month.get_start_date()?.format("%Y-%m-%d").to_string();
  let date_last = month.get_end_date()?.format("%Y-%m-%d").to_string();

  println!("Valid Date");

  let mut page_token: Option<String> = None;
  let mut transactions = Vec::new();

  fetch_payments(
    &token,
    &date_first,
    &date_last,
    &mut page_token,
    &mut transactions,
  );

  while page_token.is_some() {
    fetch_payments(
      &token,
      &date_first,
      &date_last,
      &mut page_token,
      &mut transactions,
    );
  }

  Some(transactions)
}

fn fetch_payments(
  token: &TinkToken,
  date_gte: &String,
  date_lte: &String,
  page_token: &mut Option<String>,
  transactions: &mut Vec<TinkTransaction>,
) -> Option<()> {
  let mut request = minreq::get("https://api.tink.com/data/v2/transactions")
    .with_header("Authorization", format!("Bearer {}", token.token))
    .with_param("bookedDateGte", date_gte)
    .with_param("bookedDateLte", date_lte)
    .with_param("pageSize", "100");

  if let Some(page_token) = page_token {
    request = request.with_param("pageToken", page_token.to_owned());
  }

  let response = request.send().ok()?;

  // println!("body: {}", response.as_str().unwrap_or("<ERROR IN BODY>"));

  let response = response.json::<TinkPaymentResponse>();

  if let Err(err) = response {
    println!("SERDE ERROR {}", err);
    return None;
  }

  let response = response.unwrap();

  let mut tink_transactions = response.transactions;

  // println!("transactions: {:?}", tink_transactions);

  if !response.next_page_token.is_empty() {
    *page_token = Some(response.next_page_token);
  } else {
    *page_token = None;
  }

  transactions.append(&mut tink_transactions);

  Some(())
}
