use std::{collections::HashMap, fs::read_to_string, io::Write};

use chrono::{Local, NaiveDate, NaiveTime};
use data::Database;
use serde::Deserialize;

use crate::{
    tables::{
        category::{CategoryGroupInsert, CategoryInsert},
        payment::{PaymentCategoryLinkInsert, PaymentInsert, PaymentUserLinkInsert},
        rule::{Rule, RuleCategoryLinkInsert, RuleInsert, RuleKeywordInsert},
        tink_token::TinkPayment,
        user::{User, UserInsert},
    },
    web::{convert_to_username, get_auth_hash},
};

pub fn migrate(database: &Database) {
    if !database.get_all::<User>().unwrap().is_empty() {
        panic!("database is not empty");
    }
    println!("Migrating from petra-py");

    let categories = read_category(database);
    let users = read_users(database);

    read_payment(database, &users, &categories);

    read_rule(database, &categories);
}

fn read_file(path: &str) -> String {
    let path = format!("old-data/{}.json", path);
    read_to_string(&path).expect(&format!("could not read {}", path))
}

fn parse_file<'a, T: Deserialize<'a>>(content: &'a str) -> HashMap<String, T> {
    serde_json::from_str(content).expect("could not parse")
}

#[derive(Deserialize)]
struct MigrateUser {
    displayname: String,
}

fn read_users(database: &Database) -> HashMap<String, i64> {
    let map = parse_file::<MigrateUser>(&read_file("user"));
    let mut db_map = HashMap::new();

    for (id, user) in map {
        let display_name = user.displayname;
        println!("Adding user '{}'", display_name);

        print!("New password for {}: ", display_name);
        std::io::stdout().flush().unwrap();
        let mut password = String::new();
        std::io::stdin().read_line(&mut password).unwrap();
        let password = password.strip_suffix('\n').unwrap();
        println!("password: '{}'", password);

        let name = convert_to_username(&display_name);

        let auth_hash = get_auth_hash(&name, &password);

        let user_id = database
            .insert(UserInsert {
                name,
                display_name,
                auth_hash,
            })
            .expect("could not add user");

        db_map.insert(id, user_id);
        println!("Added user with id {}", user_id);
    }

    return db_map;
}

#[derive(Deserialize, Clone)]
struct MigrateCategoryGroup {
    name: String,
    icon: String,
}

#[derive(Deserialize, Clone)]
struct MigrateCategory {
    name: String,
    icon: String,
    group: String,
}

fn read_category(database: &Database) -> HashMap<String, i64> {
    let group_map = parse_file::<MigrateCategoryGroup>(&read_file("category_group"));
    let mut group_db_map = HashMap::new();

    for (group_id, group) in group_map {
        let id = database
            .insert(CategoryGroupInsert {
                name: group.name.clone(),
                icon: group.icon,
            })
            .expect(&format!("could not add category group"));

        println!("Added category group '{}' ({})", group.name, group_id);
        group_db_map.insert(group_id, id);
    }

    let category_map = parse_file::<MigrateCategory>(&read_file("category"));
    let mut category_db_map: HashMap<String, i64> = HashMap::new();

    for (category_id, category) in category_map {
        let group_id = group_db_map
            .get(&category.group)
            .expect(&format!("could not get category group {}", category.group))
            .to_owned();

        let id = database
            .insert(CategoryInsert {
                name: category.name.clone(),
                icon: category.icon,
                group_id,
            })
            .expect(&format!("could not add category"));
        println!("Added category '{}' ({})", category.name, category_id);

        category_db_map.insert(category_id, id);
    }

    return category_db_map;
}

#[derive(Deserialize)]
struct MigratePayment {
    name: String,
    amount: i64,
    date: String,
    categories: Vec<String>,
    users: Vec<String>,
    tink_id: Option<String>,
    owner: String,
}

fn read_payment(
    database: &Database,
    users: &HashMap<String, i64>,
    categories: &HashMap<String, i64>,
) -> HashMap<String, i64> {
    const FILE_NAMES: [&'static str; 5] = [
        "payment.2022.09",
        "payment.2022.10",
        "payment.2022.11",
        "payment.2022.12",
        "payment.2023.01",
    ];

    let mut payment_db_map = HashMap::new();

    for file_name in FILE_NAMES {
        let map = parse_file::<MigratePayment>(&read_file(file_name));

        for (payment_str_id, payment) in map {
            let owner_id = users.get(&payment.owner).unwrap().to_owned();

            let date_parts: Vec<&str> = payment.date.split('-').collect();

            let year = date_parts[0].parse().unwrap();
            let month = date_parts[1].parse().unwrap();
            let day = date_parts[2].parse().unwrap();

            let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();

            let date = date
                .and_time(NaiveTime::MIN)
                .and_local_timezone(Local)
                .unwrap();

            let timestamp = date.to_rfc3339();

            let payment_id = database
                .insert(PaymentInsert {
                    name: payment.name,
                    amount: payment.amount,
                    original_amount: payment.amount,
                    timestamp: timestamp.clone(),
                    original_timestamp: timestamp,
                    owner_id,
                })
                .unwrap();

            for user in payment.users {
                let user_id = users.get(&user).unwrap().to_owned();

                database
                    .insert(PaymentUserLinkInsert {
                        payment_id,
                        user_id,
                    })
                    .unwrap();
            }

            for category in payment.categories {
                let category_id = categories.get(&category).unwrap().to_owned();

                database
                    .insert(PaymentCategoryLinkInsert {
                        payment_id,
                        category_id,
                    })
                    .unwrap();
            }

            if let Some(tink_transaction_hash) = payment.tink_id {
                database
                    .insert(TinkPayment {
                        payment_id,
                        tink_transaction_hash,
                    })
                    .unwrap();
            }

            payment_db_map.insert(payment_str_id, payment_id);
        }
    }

    payment_db_map
}

#[derive(Deserialize, Clone)]
struct MigrateRule {
    name: String,
    keywords: Vec<String>,
    categories: Vec<String>,
    shared: Option<bool>,
}

fn read_rule(database: &Database, categories: &HashMap<String, i64>) -> HashMap<String, i64> {
    let map = parse_file::<MigrateRule>(&read_file("rule"));

    let mut db_map = HashMap::new();

    for (rule_id, rule) in map {
        let id = database
            .insert(RuleInsert {
                name: rule.name.clone(),
                shared: Rule::get_share_num(&rule.shared),
            })
            .unwrap();

        db_map.insert(rule_id, id);
        println!("Added rule '{}' ({})", rule.name, id);

        for keyword in rule.keywords {
            database
                .insert(RuleKeywordInsert {
                    keyword,
                    rule_id: id,
                })
                .unwrap();
        }

        for category in rule.categories {
            let category_id = categories.get(&category).unwrap().to_owned();
            database
                .insert(RuleCategoryLinkInsert {
                    rule_id: id,
                    category_id,
                })
                .unwrap();
        }
    }

    db_map
}
