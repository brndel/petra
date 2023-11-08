use chrono::{DateTime, FixedOffset, Local};
use mensula::Table;
use mensula::query::SelectQuery;
use mensula_key::Key;
use tink_banking::{get_auth_token, get_transactions, AuthToken, TinkMonth, TransactionStatus};

use crate::{db::get_db, util::month::MonthDate};

use crate::api::payment::server::Payment;
use crate::api::tink::{TinkPaymentResponse, AddTinkPayment};
use crate::api::user::server::User;

#[derive(Table)]
pub struct TinkPayment {
    #[primary]
    #[foreign(Payment)]
    #[on_delete("cascade")]
    id: Key,
    name: String,
    amount: i64,
    timestamp: String,
    hash: String,
}

#[derive(Table)]
pub struct TinkToken {
    #[primary]
    #[foreign(User)]
    #[on_delete("cascade")]
    id: Key,
    token: String,
    expires_timestamp: String,
}

pub fn create_token(user: Key, auth_code: &str) -> Option<AuthToken> {
    let token = get_auth_token(auth_code);

    if let Some(token) = token {
        let tink_token = TinkToken {
            id: user,
            token: token.token.clone(),
            expires_timestamp: token.expires_timestamp.to_rfc3339(),
        };

        get_db().insert(tink_token)?;

        Some(token)
    } else {
        None
    }
}

pub fn get_token(id: Key) -> Option<AuthToken> {
    let db = get_db();

    match db.get::<TinkToken>(id.clone()) {
        Some(token) => {
            if let Some(timestamp) = get_timestamp_if_valid(&token) {
                Some(AuthToken {
                    token: token.token,
                    expires_timestamp: timestamp,
                })
            } else {
                let _ = db.delete::<TinkToken>(id);
                None
            }
        }
        None => None,
    }
}

fn get_timestamp_if_valid(token: &TinkToken) -> Option<DateTime<FixedOffset>> {
    let now = Local::now();

    let timestamp = DateTime::parse_from_rfc3339(&token.expires_timestamp).ok()?;

    if now <= timestamp {
        Some(timestamp)
    } else {
        None
    }
}

pub fn get_payments(user: Key, month: MonthDate) -> Option<TinkPaymentResponse> {
    let token = get_token(user)?;

    let month = TinkMonth {
        year: month.year,
        month: month.month.get_number() as u32,
    };

    let transactions = get_transactions(&token.token, &month).ok()?;

    let mut new_payments = Vec::new();
    let mut pending_payments = Vec::new();
    let mut added_payments = Vec::new();

    let db = get_db();

    for transaction in transactions {
        if transaction.status != TransactionStatus::Booked {
            pending_payments.push(transaction.into());
            continue;
        }

        if let Some(key) = SelectQuery::new().filter(TinkPayment::hash().eq(transaction.ref_hash.clone())).get_first::<Key>(&db) {
            added_payments.push((transaction.into(), key));
            continue;
        }

        new_payments.push(transaction.into());
    }

    Some(TinkPaymentResponse {
        new_payments,
        pending_payments,
        added_payments,
    })
}

pub fn add_tink_payment(payment_id: Key, payment: AddTinkPayment) -> Option<Key> {
    let db = get_db();

    let AddTinkPayment {
        name,
        amount,
        timestamp,
        hash
    } = payment;

    db.insert(TinkPayment {
        id: payment_id,
        name,
        amount,
        timestamp: timestamp.to_rfc3339(),
        hash,
    })
}