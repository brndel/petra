use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use tink_banking::{Counterparties, Transaction};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct TinkPayment {
    pub status: TinkPaymentStatus,
    pub name: String,
    pub raw_name: String,
    pub amount: i64,
    pub timestamp: DateTime<FixedOffset>,
    pub counterparties: Option<TinkCounterparties>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum TinkPaymentStatus {
    New,
    Pending,
    AlreadyAdded,
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
impl From<(Transaction, TinkPaymentStatus)> for TinkPayment {
    fn from(value: (Transaction, TinkPaymentStatus)) -> Self {
        let (
            Transaction {
                name,
                raw_name,
                date: timestamp,
                amount,
                counterparties,
                ..
            },
            status,
        ) = value;

        Self {
            status,
            name,
            raw_name,
            amount,
            timestamp,
            counterparties: counterparties.map(Into::into),
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
