use std::num::ParseIntError;

use data::{Database, query::SelectQuery, PrimKey};
use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::tables::tink_token::TinkPayment;

#[derive(Serialize)]
pub struct ResponsePayment {
  name: String,
  name_raw: String,
  date: String,
  amount: i64,
  ref_hash: String,
  #[serde(skip)]
  is_booked: bool
}

impl TryInto<ResponsePayment> for TinkTransaction {
  type Error = ParseIntError;

  fn try_into(self) -> Result<ResponsePayment, Self::Error> {
    let name = self.descriptions.display;
    let name_raw = self.descriptions.original;
    let date = self.dates.booked;
    let amount = self.amount.try_into()?;
    let is_booked = self.status == "BOOKED";

    // let detailed_unstructured = self.descriptions.detailed.unstructured;

    let ref_hash = digest(format!("{name}|{name_raw}|{date}|{amount}"));

    Ok(ResponsePayment {
      name,
      name_raw,
      date,
      amount,
      ref_hash,
      is_booked
    })
  }
}

impl ResponsePayment {
  pub fn is_booked(&self) -> bool {
    self.is_booked
  }

  pub fn is_listed(&self, database: &Database) -> bool {
    let q = SelectQuery::new().filter(TinkPayment::tink_transaction_hash().eq(self.ref_hash.to_owned()));
    q.get_first::<PrimKey>(database).is_some()
  }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TinkTransaction {
  // id: String,
  // account_id: String,
  amount: TinkAmount,
  descriptions: TinkDescriptions,
  dates: TinkDates,
  status: String,
  // counterparties: Option<TinkCounterparties>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TinkAmount {
  // currency_code: String,
  value: TinkAmountValue,
}

impl TryInto<i64> for TinkAmount {
  type Error = ParseIntError;

  fn try_into(self) -> Result<i64, Self::Error> {
    let unscaled = self.value.unscaled_value.parse::<i64>()?;
    let scale = 2 - self.value.scale.parse::<u32>()?;

    Ok(unscaled * 10_i64.pow(scale))
  }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TinkAmountValue {
  scale: String,
  unscaled_value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TinkDates {
  booked: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TinkDescriptions {
  // detailed: Option<TinkDetailedDescriptions>,
  display: String,
  original: String,
}

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// struct TinkDetailedDescriptions {
//   unstructured: String,
// }

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// struct TinkIdentifier {
//   provider_transaction_id: String,
// }

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// struct TinkCounterparties {
//   payer: TinkCounterpartyInfo,
//   payee: TinkCounterpartyInfo,
// }

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// struct TinkCounterpartyInfo {
//   identifiers: TinkCounterpartyIdent,
//   name: String,
// }

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// struct TinkCounterpartyIdent {
//   financial_institution: TinkCounterpartyInstitution,
// }

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// struct TinkCounterpartyInstitution {
//   account_number: String,
// }
