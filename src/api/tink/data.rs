use chrono::{DateTime, FixedOffset};
use serde::{Serialize, Deserialize};

#[cfg(feature = "ssr")]
use tink_banking::{Transaction, Counterparties};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct TinkPayment {
    pub name: String,
    pub raw_name: String,
    pub amount: i64,
    pub timestamp: DateTime<FixedOffset>,
    pub counterparties: Option<TinkCounterparties>,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct TinkCounterparties {
    pub payer: TinkCounterparty,
    pub payee: TinkCounterparty,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct TinkCounterparty {
    pub name: String,
    pub account: String,
}

#[cfg(feature = "ssr")]
impl From<Transaction> for TinkPayment {
    fn from(value: Transaction) -> Self {
        Self {
            name: value.name,
            raw_name: value.raw_name,
            amount: value.amount,
            timestamp: value.date,
            counterparties: value.counterparties.map(Into::into),
            hash: value.ref_hash,
        }
    }
}

#[cfg(feature = "ssr")]
impl From<Counterparties> for TinkCounterparties {
    fn from(value: Counterparties) -> Self {
        Self {
            payer: TinkCounterparty {
                name: value.payer.name,
                account: value.payer.account,
            },
            payee: TinkCounterparty {
                name: value.payee.name,
                account: value.payee.account,
            },
        }
    }
}

impl TinkPayment {
    pub fn get_rule_strings(&self) -> Vec<&str> {
        if let Some(counterparties) = &self.counterparties {
            vec![
                &self.name,
                &self.raw_name,
                &counterparties.payee.name,
                &counterparties.payer.name,
                &counterparties.payee.account,
                &counterparties.payer.account,
            ]
        } else {
            vec![&self.name, &self.raw_name]
        }
    }
}