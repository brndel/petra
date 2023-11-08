use std::{num::ParseIntError, str::FromStr};

use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveTime, TimeZone};
use serde::{Deserialize, Serialize};

use crate::DATE_FORMAT;

use super::api_transaction::{ApiTransaction, self};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub name: String,
    pub raw_name: String,
    pub date: DateTime<FixedOffset>,
    pub amount: i64,
    pub status: TransactionStatus,
    pub ref_hash: String,
    pub counterparties: Option<Counterparties>,
}

#[derive(Debug)]
pub enum TransactionError {
    Date,
    DateParse(chrono::ParseError),
    Amount(ParseIntError),
    Status(TransactionStatusParseError),
    Counterparty,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionStatus {
    Undefined,
    Pending,
    Booked,
}

#[derive(Debug)]
pub struct TransactionStatusParseError(String);

impl FromStr for TransactionStatus {
    type Err = TransactionStatusParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "UNDEFINED" => Self::Undefined,
            "PENDING" => Self::Pending,
            "BOOKED" => Self::Booked,
            _ => return Err(TransactionStatusParseError(s.to_owned())),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Counterparties {
    pub payer: Counterparty,
    pub payee: Counterparty,
}

impl TryFrom<api_transaction::Counterparties> for Counterparties {
    type Error = TransactionError;

    fn try_from(value: api_transaction::Counterparties) -> Result<Self, Self::Error> {
        Ok(Self {
            payer: value.payer.ok_or(TransactionError::Counterparty)?.try_into()?,
            payee: value.payee.ok_or(TransactionError::Counterparty)?.try_into()?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Counterparty {
    pub name: String,
    pub account: String,
}

impl TryFrom<api_transaction::Counterparty> for Counterparty {
    type Error = TransactionError;

    fn try_from(value: api_transaction::Counterparty) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            account: value.identifiers.ok_or(TransactionError::Counterparty)?.financial_institution.account_number,
        })
    }
}

impl TryFrom<ApiTransaction> for Transaction {
    type Error = TransactionError;

    fn try_from(value: ApiTransaction) -> Result<Self, Self::Error> {
        let name = value.descriptions.display;
        let raw_name = value.descriptions.original;
        let date_str = value.dates.booked;

        let date = NaiveDate::parse_from_str(&date_str, DATE_FORMAT)
            .map_err(TransactionError::DateParse)?
            .and_time(NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap());
        let date = Local
            .from_local_datetime(&date)
            .earliest()
            .ok_or(TransactionError::Date)?
            .fixed_offset();

        let amount = value.amount.try_into().map_err(TransactionError::Amount)?;

        let status = value.status.parse().map_err(TransactionError::Status)?;

        let ref_hash = sha256::digest(format!("{name}|{raw_name}|{date_str}|{amount}"));

        let counterparties = match value.counterparties {
            Some(counterparties) => Some(counterparties.try_into()?),
            None => None,
        };

        Ok(Self {
            name,
            raw_name,
            date,
            amount,
            status,
            ref_hash,
            counterparties,
        })
    }
}
