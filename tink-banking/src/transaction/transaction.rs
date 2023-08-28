use std::{num::ParseIntError, str::FromStr};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::DATE_FORMAT;

use super::response_transaction::ResponseTransaction;

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    name: String,
    raw_name: String,
    date: NaiveDate,
    amount: i64,
    status: TransactionStatus,
    ref_hash: String,
}

#[derive(Serialize, Deserialize)]
pub enum TransactionStatus {
    Undefined,
    Pending,
    Booked,
}

impl FromStr for TransactionStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "UNDEFINED" => Self::Undefined,
            "PENDING" => Self::Pending,
            "BOOKED" => Self::Booked,
            _ => return Err(()),
        })
    }
}

pub enum TransactionError {
    Date(chrono::ParseError),
    Amount(ParseIntError),
    Status,
}

impl TryFrom<ResponseTransaction> for Transaction {
    type Error = TransactionError;

    fn try_from(value: ResponseTransaction) -> Result<Self, Self::Error> {
        let name = value.descriptions.display;
        let raw_name = value.descriptions.original;
        let date_str = value.dates.booked;
        let date =
            NaiveDate::parse_from_str(&date_str, DATE_FORMAT).map_err(TransactionError::Date)?;
        let amount = value.amount.try_into().map_err(TransactionError::Amount)?;
        let status = value.status.parse().map_err(|_| TransactionError::Status)?;

        let ref_hash = sha256::digest(format!("{name}|{raw_name}|{date_str}|{amount}"));

        Ok(Self {
            name,
            raw_name,
            date,
            amount,
            status,
            ref_hash,
        })
    }
}
