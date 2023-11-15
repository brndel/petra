use chrono::{DateTime, FixedOffset, Local};
use leptos::{RwSignal, SignalGet, SignalGetUntracked, SignalWith, SignalWithUntracked};
use mensula_key::Key;

use crate::{
    api::{
        payment::AddPaymentData,
        rule::{Rule, ShareRule},
        tink::{TinkPaymentData, TinkPayment, TinkPaymentStatus},
    },
    util::calculated_amount::CalculatedAmount,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EditPayment {
    pub enabled: RwSignal<bool>,
    pub name: RwSignal<String>,
    pub amount: RwSignal<Option<i64>>,
    pub date: RwSignal<Option<DateTime<FixedOffset>>>,
    pub categories: RwSignal<Vec<Key>>,
    pub users: RwSignal<Vec<Key>>,
    pub import_data: Option<ImportData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportData {
    pub tink: TinkPayment,
    pub rule: Option<Key>,
}

impl EditPayment {
    pub fn new() -> Self {
        Self {
            enabled: RwSignal::new(true),
            name: RwSignal::new(String::new()),
            amount: RwSignal::new(Some(0)),
            date: RwSignal::new(Some(Local::now().fixed_offset())),
            categories: RwSignal::new(Vec::new()),
            users: RwSignal::new(Vec::new()),
            import_data: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.enabled.get()
            || self.amount.with(|amount| amount.is_some())
                && self.date.with(|date| date.is_some())
                && AddPaymentData::is_valid_static(&self.name.get(), &self.users.get())
    }

    pub fn is_valid_untracked(&self) -> bool {
        !self.enabled.get()
            || self.amount.with_untracked(|amount| amount.is_some())
                && self.date.with_untracked(|date| date.is_some())
                && AddPaymentData::is_valid_static(
                    &self.name.get_untracked(),
                    &self.users.get_untracked(),
                )
    }

    pub fn from_tink_payment(
        payment: TinkPayment,
        rules: &[Rule],
        users: &[Key],
        me: &Key,
    ) -> Self {
        let rule = Rule::find_rule(rules, &payment);

        let users = match rule.as_ref().map(|rule| rule.share_rule.clone()) {
            Some(ShareRule::Shared) => users.to_owned(),
            Some(ShareRule::NotShared) => vec![me.to_owned()],

            None | Some(ShareRule::Choose) => Vec::new(),
        };

        Self {
            enabled: RwSignal::new(payment.status == TinkPaymentStatus::New),
            name: RwSignal::new(
                rule.as_ref()
                    .map(|rule| rule.name.clone())
                    .unwrap_or_default(),
            ),
            amount: RwSignal::new(Some(payment.amount)),
            date: RwSignal::new(Some(payment.timestamp)),
            categories: RwSignal::new(
                rule.as_ref()
                    .map(|rule| rule.categories.clone())
                    .unwrap_or_default(),
            ),
            users: RwSignal::new(users),
            import_data: Some(ImportData {
                tink: payment,
                rule: rule.map(|rule| rule.id.clone()),
            }),
        }
    }

    pub fn get_amount(&self, me: &Key) -> CalculatedAmount {
        CalculatedAmount::calculate(
            self.amount.get().unwrap_or_default(),
            true,
            self.users.with(|users| users.contains(&me)),
            self.users.with(|users| users.len()),
        )
    }
}

#[derive(Debug)]
pub enum EditPaymentError {
    Disabled,
    InvalidAmount,
    InvalidDate,
}

impl TryFrom<&EditPayment> for AddPaymentData {
    type Error = EditPaymentError;

    fn try_from(value: &EditPayment) -> Result<Self, Self::Error> {
        if !value.enabled.get_untracked() {
            return Err(EditPaymentError::Disabled);
        }

        Ok(Self {
            name: value.name.get_untracked(),
            amount: value
                .amount
                .get_untracked()
                .ok_or(EditPaymentError::InvalidAmount)?,
            timestamp: value
                .date
                .get_untracked()
                .ok_or(EditPaymentError::InvalidAmount)?,
            users: value.users.get_untracked(),
            categories: value.categories.get_untracked(),
            tink: value.import_data.as_ref().map(|data| TinkPaymentData {
                name: data.tink.name.clone(),
                amount: data.tink.amount,
                timestamp: data.tink.timestamp,
            }),
        })
    }
}
