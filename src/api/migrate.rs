use std::{collections::HashMap, path::Path};

use chrono::DateTime;
use mensula::{query::SelectQuery, Database, Table};
use mensula_key::Key;

use crate::{
    api::{
        category::server::{insert_category, insert_category_group},
        payment::{server::add_payment, AddPaymentData},
        rule::ShareRule,
        rule::server::insert_rule,
        tink::AddTinkPayment,
        user::server::User as NewUser,
    },
    db,
};

pub fn migrate<T: AsRef<Path>>(old_file: T) {
    let old_db = &mut Database::open(old_file).unwrap();

    let user_map = migrate_users(old_db);
    let category_map = migrate_categories(old_db);
    let tink_payment_map = get_tink_payments(old_db);
    let _payment_map = migrate_payments(old_db, &user_map, &category_map, &tink_payment_map);
    let _rule_map = migrate_rules(old_db, &category_map);
}

#[derive(Table)]
#[table_name("User")]
struct OldUser {
    #[primary]
    id: i64,
    name: String,
    display_name: String,
    auth_hash: String,
}

fn migrate_users(old_db: &mut Database) -> HashMap<i64, Key> {
    println!("Registering OldUser {:?}", old_db.register::<OldUser>());

    let old_users = old_db.get_all::<OldUser>().unwrap();

    let mut user_map = HashMap::new();

    let mut new_users = Vec::new();

    for user in old_users {
        println!("migrating user '{}'", user.name);
        let password = rpassword::prompt_password("please input password for user:").unwrap();

        let new_user = NewUser::create(user.name, user.display_name, password);

        match new_user {
            Ok(new_user) => {
                user_map.insert(user.id, new_user.id.clone());
                new_users.push(new_user);
            }
            Err(err) => panic!("{:?}", err),
        }
    }

    let new_db = db::get_db();

    for user in new_users {
        new_db.insert(user);
    }

    user_map
}

#[derive(Table)]
#[table_name("CategoryGroup")]
struct OldCategoryGroup {
    #[primary]
    id: i64,
    name: String,
    icon: String,
}

#[derive(Table)]
#[table_name("Category")]
struct OldCategory {
    #[primary]
    id: i64,
    name: String,
    icon: String,
    group_id: i64,
}

fn migrate_categories(old_db: &mut Database) -> HashMap<i64, Key> {
    println!(
        "Registering OldCategoryGroup {:?}",
        old_db.register::<OldCategoryGroup>()
    );

    let old_category_groups = old_db.get_all::<OldCategoryGroup>().unwrap();

    let mut category_group_map = HashMap::new();

    for group in old_category_groups {
        let id = insert_category_group(None, group.name, group.icon).unwrap();

        category_group_map.insert(group.id, id);
    }

    let old_categories = old_db.get_all::<OldCategory>().unwrap();

    let mut category_map = HashMap::new();

    for category in old_categories {
        let group = category_group_map[&category.group_id].clone();

        let id = insert_category(None, category.name, category.icon, group).unwrap();

        category_map.insert(category.id, id);
    }

    category_map
}

#[derive(Table)]
#[table_name("Payment")]
struct OldPayment {
    #[primary]
    id: i64,
    name: String,
    amount: i64,
    timestamp: String,
    owner_id: i64,
}

#[derive(Table)]
#[table_name("PaymentCategoryLink")]
struct OldPaymentCategoryLink {
    #[primary]
    id: i64,
    payment_id: i64,
    category_id: i64,
}

#[derive(Table)]
#[table_name("PaymentUserLink")]
struct OldPaymentUserLink {
    #[primary]
    id: i64,
    payment_id: i64,
    user_id: i64,
}

#[derive(Table)]
#[table_name("TinkPayment")]
struct OldTinkPayment {
    #[primary]
    payment_id: i64,
    tink_transaction_hash: String,
}

fn get_tink_payments(old_db: &mut Database) -> HashMap<i64, String> {
    old_db.register::<OldTinkPayment>().unwrap();

    let mut map = HashMap::new();

    for payment in old_db.get_all::<OldTinkPayment>().unwrap() {
        map.insert(payment.payment_id, payment.tink_transaction_hash);
    }

    map
}

fn migrate_payments(
    old_db: &mut Database,
    user_map: &HashMap<i64, Key>,
    category_map: &HashMap<i64, Key>,
    tink_payment_map: &HashMap<i64, String>,
) -> HashMap<i64, Key> {
    println!("migrating payments");
    old_db.register::<OldPayment>().unwrap();
    old_db.register::<OldPaymentCategoryLink>().unwrap();
    old_db.register::<OldPaymentUserLink>().unwrap();

    let mut payment_map = HashMap::new();

    for old_payment in old_db.get_all::<OldPayment>().unwrap() {
        let owner = user_map[&old_payment.owner_id].clone();
        let timestamp = DateTime::parse_from_rfc3339(&old_payment.timestamp).unwrap();

        let categories = SelectQuery::new()
            .filter(OldPaymentCategoryLink::payment_id().eq(old_payment.id))
            .get_all::<OldPaymentCategoryLink>(&old_db)
            .unwrap();
        let users = SelectQuery::new()
            .filter(OldPaymentUserLink::payment_id().eq(old_payment.id))
            .get_all::<OldPaymentUserLink>(&old_db)
            .unwrap();

        let categories = categories
            .into_iter()
            .map(|category| category_map[&category.category_id].clone())
            .collect();
        let users = users
            .into_iter()
            .map(|user| user_map[&user.user_id].clone())
            .collect();

        let tink = tink_payment_map
            .get(&old_payment.id)
            .map(|hash| AddTinkPayment {
                name: "???".to_owned(),
                amount: old_payment.amount,
                timestamp: timestamp.clone(),
                hash: hash.clone(),
            });

        let id = add_payment(
            owner,
            AddPaymentData {
                name: old_payment.name,
                amount: old_payment.amount,
                timestamp,
                categories,
                users,
                tink,
            },
        )
        .unwrap();

        payment_map.insert(old_payment.id, id);
    }

    payment_map
}

#[derive(Table)]
#[table_name("Rule")]
struct OldRule {
    #[primary]
    id: i64,
    name: String,
    shared: i64,
}

#[derive(Table)]
#[table_name("RuleCategoryLink")]
struct OldRuleCategoryLink {
    #[primary]
    id: i64,
    rule_id: i64,
    category_id: i64,
}

#[derive(Table)]
#[table_name("RuleKeyword")]
struct OldRuleKeyword {
    #[primary]
    id: i64,
    keyword: String,
    rule_id: i64,
}

fn migrate_rules(old_db: &mut Database, category_map: &HashMap<i64, Key>) -> HashMap<i64, Key> {
    old_db.register::<OldRule>().unwrap();
    old_db.register::<OldRuleKeyword>().unwrap();
    old_db.register::<OldRuleCategoryLink>().unwrap();

    let mut rule_map = HashMap::new();

    for old_rule in old_db.get_all::<OldRule>().unwrap() {
        let OldRule { id, name, shared } = old_rule;

        let shared = ShareRule::try_from(shared).unwrap();

        let keywords = SelectQuery::new()
            .filter(OldRuleKeyword::rule_id().eq(id))
            .get_all::<OldRuleKeyword>(&old_db)
            .unwrap();
        let keywords = keywords
            .into_iter()
            .map(|keyword| keyword.keyword)
            .collect();

        let categories = SelectQuery::new()
            .filter(OldRuleCategoryLink::rule_id().eq(id))
            .get_all::<OldRuleCategoryLink>(&old_db)
            .unwrap();
        let categories = categories
            .into_iter()
            .map(|category| category_map.get(&category.category_id).unwrap().clone())
            .collect();

        let old_id = id;
        let id = insert_rule(None, name, shared, keywords, categories).unwrap();

        rule_map.insert(old_id, id);
    }

    rule_map
}
