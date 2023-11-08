use serde::Deserialize;

use crate::{month::TinkMonth, transaction::api_transaction::ApiTransaction, DATE_FORMAT};

use super::Transaction;

static TRANSACTIONS_URL: &str = "https://api.tink.com/data/v2/transactions";

pub enum TransactionError {
    BadMonth,
    Parsing,
}

#[derive(Default)]
struct TransactionsFetchData {
    page_token: Option<String>,
    transactions: Vec<Transaction>,
}

pub fn get_transactions(
    auth_token: &str,
    month: &TinkMonth,
) -> Result<Vec<Transaction>, TransactionError> {
    let first_day = month
        .get_first_day()
        .ok_or(TransactionError::BadMonth)?
        .format(DATE_FORMAT)
        .to_string();
    let last_day = month
        .get_last_day()
        .ok_or(TransactionError::BadMonth)?
        .format(DATE_FORMAT)
        .to_string();

    let mut fetch_data = TransactionsFetchData::default();

    fetch_transactions(auth_token, &first_day, &last_day, &mut fetch_data);

    while fetch_data.page_token.is_some() {
        fetch_transactions(auth_token, &first_day, &last_day, &mut fetch_data);
    }

    Ok(fetch_data.transactions)
}

fn fetch_transactions(
    auth_token: &str,
    first_day: &str,
    last_day: &str,
    fetch_data: &mut TransactionsFetchData,
) -> Option<()> {
    let mut request = minreq::get(TRANSACTIONS_URL)
        .with_header("Authorization", format!("Bearer {}", auth_token))
        .with_param("bookedDateGte", first_day)
        .with_param("bookedDateLte", last_day)
        .with_param("pageSize", "100");

    if let Some(page_token) = &fetch_data.page_token {
        request = request.with_param("pageToken", page_token);
    }

    let response = request.send().ok()?;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        next_page_token: String,
        transactions: Vec<ApiTransaction>,
    }

    let response: Response = match response.json() {
        Ok(response) => response,
        Err(err) => {dbg!(err); return None}
    };

    fetch_data.page_token = if response.next_page_token.is_empty() {
        None
    } else {
        Some(response.next_page_token)
    };

    let mut transactions = response
        .transactions
        .into_iter()
        .filter_map(|transaction| transaction.try_into().ok())
        .collect();

    fetch_data.transactions.append(&mut transactions);

    Some(())
}
