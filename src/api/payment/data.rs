use chrono::{DateTime, FixedOffset};
use mensula_key::Key;
use serde::{Serialize, Deserialize};

use crate::util::{month::MonthDate, calculated_amount::CalculatedAmount};

#[derive(Serialize, Deserialize, Clone)]
pub struct Payment {
    pub id: Key,
    pub name: String,
    pub amount: i64,
    pub timestamp: DateTime<FixedOffset>,
    pub owner: Key,
    pub users: Vec<Key>,
    pub categories: Vec<Key>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PaymentMonthData {
    pub month: MonthDate,
    pub payments_count: u32,
    pub amount: CalculatedAmount,
}

impl From<MonthDate> for PaymentMonthData {
    fn from(value: MonthDate) -> Self {
        Self {
            month: value,
            payments_count: 0,
            amount: CalculatedAmount::default(),
        }
    }
}

impl Payment {
    pub fn get_amount(&self, user: &Key) -> CalculatedAmount {
        CalculatedAmount::new(user, &self)
    }

    pub fn get_all_amounts(&self) -> Vec<(Key, CalculatedAmount)> {
        let user_count = self.users.len();

        self.users
            .iter()
            .map(|user| {
                (
                    user.clone(),
                    CalculatedAmount::calculate(self.amount, user == &self.owner, true, user_count),
                )
            })
            .collect()
    }
}