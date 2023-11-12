use chrono::{DateTime, FixedOffset};
use leptos::{ServerFnError, server};
use mensula_key::Key;
use serde::{Serialize, Deserialize};

use crate::{util::{month::MonthDate, calculated_amount::CalculatedAmount}, api::tink::TinkPaymentData};

pub use super::data::*;

#[cfg(feature = "ssr")]
use super::server;


pub struct PaymentFetchError;

impl From<PaymentFetchError> for ServerFnError {
    fn from(_: PaymentFetchError) -> Self {
        Self::ServerError("could not get payment".to_owned())
    }
}

#[server]
pub async fn get_payments(month: MonthDate) -> Result<Vec<Payment>, ServerFnError> {
    let user = crate::auth::get_user().await?;

    server::get_payments(&user, &month).map_err(Into::into)
}

#[server]
pub async fn get_months() -> Result<Vec<PaymentMonthData>, ServerFnError> {
    let user = crate::auth::get_user().await?;

    server::get_months(user).map_err(Into::into)
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AddPaymentData {
    pub name: String,
    pub amount: i64,
    pub timestamp: DateTime<FixedOffset>,
    pub users: Vec<Key>,
    pub categories: Vec<Key>,
    pub tink: Option<TinkPaymentData>,
}

impl AddPaymentData {
    pub fn is_valid_static(name: &str, users: &[Key]) -> bool { 
        name.len() >= 1 && users.len() >= 1
    }

    pub fn is_valid(&self) -> bool {
        Self::is_valid_static(&self.name, &self.users)
    }
}

#[server]
pub async fn add_payments(payments: Vec<AddPaymentData>) -> Result<(), ServerFnError> {
    let user = crate::auth::get_user().await?;

    for payment in payments {
        server::insert_payment(None, user.clone(), payment);
    }

    Ok(())
}

#[server]
pub async fn get_payment(id: Key) -> Result<Payment, ServerFnError> {
    let user = crate::auth::get_user().await?;

    server::get_payment(user, id).map_err(Into::into)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AmountResult {
    pub own: CalculatedAmount,
    pub others: Vec<(Key, CalculatedAmount)>,
}

#[server]
pub async fn calculate_all_amounts() -> Result<AmountResult, ServerFnError> {
    let user = crate::auth::get_user().await?;

    let mut amounts = server::calculate_all_amounts().map_err(ServerFnError::from)?;

    let (_, own_amount) = amounts.remove_entry(&user).ok_or_else(|| ServerFnError::from(PaymentFetchError))?;

    let others = amounts.into_iter().map(|(user, mut amount)| {
        amount.user_amount = 0;
        (user, amount)
    }).collect::<Vec<_>>();

    Ok(AmountResult { own: own_amount, others })
}