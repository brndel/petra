use std::ops::{Add, AddAssign};

use mensula_key::Key;
use serde::{Serialize, Deserialize};

use crate::api::payment::Payment;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CalculatedAmount {
    /// The amount the user really paid
    pub user_amount: i64,
    /// The amount the user should get back from the other users
    pub repay_amount: i64,
}

impl Add for CalculatedAmount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            user_amount: self.user_amount + rhs.user_amount,
            repay_amount: self.repay_amount + rhs.repay_amount,
        }
    }
}

impl AddAssign for CalculatedAmount {
    fn add_assign(&mut self, rhs: Self) {
        self.user_amount += rhs.user_amount;
        self.repay_amount += rhs.repay_amount;
    }
}

impl CalculatedAmount {
    pub fn new(user: &Key, payment: &Payment) -> Self {
        Self::calculate(
            payment.amount,
            user == &payment.owner,
            payment.users.contains(user),
            payment.users.len(),
        )
    }

    pub fn calculate(amount: i64, is_owner: bool, is_user: bool, user_count: usize) -> Self {
        let user_amount = if is_owner { amount } else { 0 };

        let mut repay_amount;

        if user_count == 0 {
            repay_amount = 0;
        } else {
            repay_amount = -user_amount;

            if is_user {
                let amount_per_user = amount / user_count as i64;
                repay_amount += amount_per_user;
            }
        }

        Self {
            user_amount,
            repay_amount,
        }
    }

    pub fn calculated_amount(&self) -> i64 {
        self.user_amount + self.repay_amount
    }
}