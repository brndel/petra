use chrono::{DateTime, FixedOffset, Local};
use mensula::query::SelectQuery;
use mensula::{Database, Table};
use mensula_key::Key;
use tink_banking::{
    get_auth_token, get_transactions, AuthToken, TinkMonth, Transaction, TransactionStatus,
};

use crate::{db::get_db, util::month::MonthDate};

use crate::api::payment::server::Payment;
use crate::api::tink::{TinkPayment as ResponseTinkPayment, TinkPaymentData};
use crate::api::user::server::User;

use super::TinkPaymentStatus;

#[derive(Table)]
pub struct TinkPayment {
    #[primary]
    #[foreign(Payment)]
    #[on_delete("cascade")]
    id: Key,
    name: String,
    amount: i64,
    timestamp: String,
    #[foreign(User)]
    #[on_delete("cascade")]
    owner: Key,
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

pub fn get_payments(user: Key, month: MonthDate) -> Option<Vec<ResponseTinkPayment>> {
    let token = get_token(user.clone())?;

    let month = TinkMonth {
        year: month.year,
        month: month.month.get_number() as u32,
    };

    let transactions = get_transactions(&token.token, &month).ok()?;

    let mut payments = Vec::new();

    let db = get_db();

    for transaction in transactions {
        let status = get_status(&transaction, &user, &db);

        payments.push((transaction, status).into());
    }

    Some(payments)
}

fn get_status(transaction: &Transaction, user: &Key, db: &Database) -> TinkPaymentStatus {
    if transaction.status != TransactionStatus::Booked {
        return TinkPaymentStatus::Pending;
    }

    let timestamp = transaction.date.date_naive().format("%Y-%m-%dT%%").to_string();

    let payment = SelectQuery::new()
        .filter(
            TinkPayment::amount()
                .eq(transaction.amount)
                .and(TinkPayment::timestamp().like(timestamp))
                .and(TinkPayment::owner().eq(user.clone())),
        )
        .get_first::<Key>(&db);

    if payment.is_some() {
        TinkPaymentStatus::AlreadyAdded
    } else {
        TinkPaymentStatus::New
    }
}

pub fn add_tink_payment(payment_id: Key, owner: Key, payment: TinkPaymentData) -> Option<Key> {
    let db = get_db();

    let TinkPaymentData {
        name,
        amount,
        timestamp,
    } = payment;

    db.insert(TinkPayment {
        id: payment_id,
        name,
        amount,
        timestamp: timestamp.to_rfc3339(),
        owner,
    })
}

pub fn get_payment_data(id: Key, db: Option<&Database>) -> Option<TinkPaymentData> {
    let mutex;
    let db = match db {
        Some(db) => db,
        None => {
            mutex = get_db();
            &mutex
        }
    };

    db.get::<TinkPayment>(id).and_then(|payment| {
        Some(TinkPaymentData {
            name: payment.name,
            amount: payment.amount,
            timestamp: DateTime::parse_from_rfc3339(&payment.timestamp).ok()?,
        })
    })
}

pub fn get_tink_url() -> String {
    tink_banking::get_url()
}