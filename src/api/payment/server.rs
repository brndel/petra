use std::collections::{BTreeMap, HashMap};

use chrono::DateTime;
use mensula::query::{Ordering, SelectQuery};
use mensula::{Database, Filter, Table};
use mensula_key::Key;

use crate::api::tink;
use crate::api::{category::server::Category, tink::server::add_tink_payment, user::server::User};
use crate::db::get_db;
use crate::util::calculated_amount::CalculatedAmount;
use crate::util::month::MonthDate;

use super::api::{AddPaymentData, Payment as ResponsePayment, PaymentFetchError, PaymentUpdateError, PaymentMonthData};

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
    let users = SelectQuery::<User>::link::<Payment, PaymentUserLink>(payment.id.clone())
        .order_by(User::name(), Ordering::Ascending)
        .get_all(&db)
        .ok_or(PaymentFetchError)?;
    let categories =
        SelectQuery::<Category>::link::<Payment, PaymentCategoryLink>(payment.id.clone())
            .order_by(Category::name(), Ordering::Ascending)
            .get_all(&db)
            .ok_or(PaymentFetchError)?;

    let tink_payment = tink::server::get_payment_data(payment.id.clone(), Some(db));

    Ok(ResponsePayment {
        id: payment.id,
        name: payment.name,
        amount: payment.amount,
        timestamp: DateTime::parse_from_rfc3339(&payment.timestamp)
            .map_err(|_| PaymentFetchError)?,
        owner: payment.owner,
        users,
        categories,
        imported: tink_payment.is_some(),
    })
}

fn user_filter(user: &Key) -> Filter<Payment> {
    Payment::owner()
        .eq(user.clone())
        .or(Payment::id().link::<PaymentUserLink, User>(user.clone()))
}

pub fn get_payment(user: Key, id: Key) -> Result<ResponsePayment, PaymentFetchError> {
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

        let users = SelectQuery::<User>::link::<Payment, PaymentUserLink>(payment.id)
            .get_all(&db)
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

pub fn calculate_all_amounts() -> Result<HashMap<Key, CalculatedAmount>, PaymentFetchError> {
    let db = get_db();

    let users = SelectQuery::<User>::new()
        .get_all::<Key>(&db)
        .ok_or(PaymentFetchError)?;

    let mut users: HashMap<Key, CalculatedAmount> = HashMap::from_iter(
        users
            .into_iter()
            .map(|user| (user, CalculatedAmount::default())),
    );

    let payments = SelectQuery::<Payment>::new()
        .get_all::<Payment>(&db)
        .ok_or(PaymentFetchError)?;

    for payment in payments {
        let payment_users = SelectQuery::<User>::link::<Payment, PaymentUserLink>(payment.id)
            .get_all::<Key>(&db)
            .ok_or(PaymentFetchError)?;
        let user_count = payment_users.len();

        let mut owner_added = false;

        for user in payment_users {
            let is_owner = user == payment.owner;
            owner_added = owner_added || is_owner;

            users.entry(user).and_modify(|amount| {
                *amount += CalculatedAmount::calculate(
                    payment.amount,
                    is_owner,
                    true,
                    user_count,
                );
            });
        }

        if !owner_added {
            users.entry(payment.owner).and_modify(|amount| {
                *amount += CalculatedAmount::calculate(payment.amount, true, false, user_count);
            });
        }
    }

    Ok(users)
}

pub fn insert_payment(id: Option<Key>, owner: Key, payment: AddPaymentData) -> Option<Key> {
    if !payment.is_valid() {
        return None;
    }

    let id = id.unwrap_or_else(Key::new);

    let server_payment = Payment {
        id,
        name: payment.name,
        amount: payment.amount,
        timestamp: payment.timestamp.to_rfc3339(),
        owner: owner.clone(),
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
        add_tink_payment(payment_id.clone(), owner, tink_payment);
    }

    Some(payment_id)
}


pub fn payment_update_users(request_user: Key, payment_id: Key, users: Vec<Key>) -> Result<(), PaymentUpdateError> {
    let db = get_db();

    let payment = db.get::<Payment>(payment_id.clone()).ok_or(PaymentUpdateError)?;

    if payment.owner != request_user {
        return Err(PaymentUpdateError);
    }

    let current_links = SelectQuery::<PaymentUserLink>::new().filter(PaymentUserLink::payment().eq(payment_id.clone())).get_all::<Key>(&db).ok_or(PaymentUpdateError)?;

    for key in current_links {
        let _ = db.delete::<PaymentUserLink>(key);
    }

    for user in users {
        db.insert(PaymentUserLink {
            id: Key::new(),
            payment: payment_id.clone(),
            user,
        }).ok_or(PaymentUpdateError)?;
    }

    Ok(())
}