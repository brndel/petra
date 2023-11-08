use std::collections::BTreeMap;

use chrono::DateTime;
use mensula::query::{Ordering, SelectQuery};
use mensula::{Database, Filter, Table};
use mensula_key::Key;

use crate::api::{tink::server::add_tink_payment, category::server::Category, user::server::User};
use crate::db::get_db;
use crate::util::calculated_amount::CalculatedAmount;
use crate::util::month::MonthDate;

use super::api::{
    Payment as ResponsePayment, PaymentFetchError, PaymentMonthData, AddPaymentData,
};

#[derive(Table)]
pub struct Payment {
    #[primary]
    id: Key,
    name: String,
    amount: i64,
    timestamp: String,
    #[foreign(User)]
    #[on_delete("cascade")]
    owner: Key,
}

#[derive(Table)]
pub struct PaymentUserLink {
    #[primary]
    id: Key,
    #[foreign_link(Payment)]
    payment: Key,
    #[foreign_link(User)]
    user: Key,
}

#[derive(Table)]
pub struct PaymentCategoryLink {
    #[primary]
    id: Key,
    #[foreign_link(Payment)]
    payment: Key,
    #[foreign_link(Category)]
    category: Key,
}

fn to_response_payment(
    payment: Payment,
    db: &Database,
) -> Result<ResponsePayment, PaymentFetchError> {
    let users = db
        .get_linked::<User, Key, Payment, PaymentUserLink>(payment.id.clone())
        .ok_or(PaymentFetchError)?;
    let categories = db
        .get_linked::<Category, Key, Payment, PaymentCategoryLink>(payment.id.clone())
        .ok_or(PaymentFetchError)?;
    Ok(ResponsePayment {
        id: payment.id,
        name: payment.name,
        amount: payment.amount,
        timestamp: DateTime::parse_from_rfc3339(&payment.timestamp)
            .map_err(|_| PaymentFetchError)?,
        owner: payment.owner,
        users,
        categories,
    })
}

fn user_filter(user: &Key) -> Filter<Payment> {
    Payment::owner()
        .eq(user.clone())
        .or(Payment::id().link::<PaymentUserLink, User>(user.clone()))
}

pub fn get_payment(
    user: Key,
    id: Key
) -> Result<ResponsePayment, PaymentFetchError> {
    let db = get_db();

    let payment = db.get::<Payment>(id).ok_or(PaymentFetchError)?;

    let payment = to_response_payment(payment, &db)?;

    if payment.owner == user || payment.users.contains(&user) {
        Ok(payment)
    } else {
        Err(PaymentFetchError)
    }
}

pub fn get_payments(
    user: &Key,
    month: &MonthDate,
) -> Result<Vec<ResponsePayment>, PaymentFetchError> {
    let db = get_db();

    let payments = SelectQuery::new()
        .filter(
            Payment::timestamp()
                .like(format!("{}-%", month))
                .and(user_filter(&user)),
        )
        .order_by(Payment::timestamp(), Ordering::Descending)
        .get_all::<Payment>(&db)
        .ok_or(PaymentFetchError)?;

    let mut response_payments = Vec::new();

    for payment in payments {
        response_payments.push(to_response_payment(payment, &db)?)
    }

    Ok(response_payments)
}

pub fn get_months(user: Key) -> Result<Vec<PaymentMonthData>, PaymentFetchError> {
    let db = get_db();

    let payments = SelectQuery::new()
        .filter(user_filter(&user))
        .get_all::<Payment>(&db)
        .ok_or(PaymentFetchError)?;

    let mut months = BTreeMap::<MonthDate, PaymentMonthData>::new();

    for payment in payments {
        let month: MonthDate = payment.timestamp.parse().map_err(|_| PaymentFetchError)?;

        let users = db
            .get_linked::<User, Key, Payment, PaymentUserLink>(payment.id)
            .ok_or(PaymentFetchError)?;
        let is_owner = payment.owner == user;
        let is_user = users.contains(&user);

        let amount = CalculatedAmount::calculate(payment.amount, is_owner, is_user, users.len());

        let month_data = months
            .entry(month.clone())
            .or_insert_with(|| PaymentMonthData::from(month));

        month_data.payments_count += 1;
        month_data.amount += amount;
    }

    Ok(months.into_values().rev().collect())
}

pub fn add_payment(owner: Key, payment: AddPaymentData) -> Option<Key> {
    if !payment.is_valid() {
        return None;
    }

    let server_payment = Payment {
        id: Key::new(),
        name: payment.name,
        amount: payment.amount,
        timestamp: payment.timestamp.to_rfc3339(),
        owner,
    };

    let db = get_db();

    let payment_id = db.insert(server_payment)?;

    for category in payment.categories {
        db.insert(PaymentCategoryLink {
            id: Key::new(),
            payment: payment_id.clone(),
            category,
        });
    }

    for user in payment.users {
        db.insert(PaymentUserLink {
            id: Key::new(),
            payment: payment_id.clone(),
            user,
        });
    }

    drop(db);

    if let Some(tink_payment) = payment.tink {
        add_tink_payment(payment_id.clone(), tink_payment);
    }

    Some(payment_id)
}
