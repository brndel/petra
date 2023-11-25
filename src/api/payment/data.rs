use chrono::{DateTime, FixedOffset};
use mensula_key::Key;
use serde::{Deserialize, Serialize};

use crate::util::{calculated_amount::CalculatedAmount, month::MonthDate};

#[derive(Serialize, Deserialize, Clone)]
pub struct Payment {
    pub id: Key,
    pub name: String,
    pub amount: i64,
    pub timestamp: DateTime<FixedOffset>,
    pub owner: Key,
    pub users: Vec<Key>,
    pub categories: Vec<Key>,
    pub imported: bool,
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

        let mut owner_added = false;

        let mut amounts = self.users
            .iter()
            .map(|user| {
                (user.clone(), {
                    let is_owner = user == &self.owner;
                    owner_added = owner_added || is_owner;

                    CalculatedAmount::calculate(self.amount, is_owner, true, user_count)
                })
            })
            .collect::<Vec<_>>();

        if !owner_added {
            amounts.push((self.owner.clone(), CalculatedAmount::calculate(self.amount, true, false, user_count)));
        }

        amounts
    }
}
