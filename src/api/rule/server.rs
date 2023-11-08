use mensula::query::{SelectQuery, Ordering};
use mensula::{Database, Table};
use mensula_key::Key;

use crate::api::category::server::Category;
use crate::db::get_db;

use super::api::{Rule as ResponseRule, RuleFetchError, ShareRule, RuleInsertError};

#[derive(Table)]
pub struct Rule {
    #[primary]
    id: Key,
    name: String,
    share_rule: i64,
}

#[derive(Table)]
pub struct RuleCategoryLink {
    #[primary]
    id: Key,
    #[foreign_link(Rule)]
    rule: Key,
    #[foreign_link(Category)]
    category: Key,
}

#[derive(Table)]
pub struct RuleKeyword {
    #[primary]
    id: Key,
    #[foreign(Rule)]
    rule: Key,
    keyword: String,
}

fn to_response_rule(rule: Rule, db: &Database) -> Result<ResponseRule, RuleFetchError> {
    let categories = db
        .get_linked::<Category, Key, Rule, RuleCategoryLink>(rule.id.clone())
        .ok_or(RuleFetchError)?;
    let keywords = SelectQuery::new()
        .filter(RuleKeyword::rule().eq(rule.id.clone()))
        .get_all::<RuleKeyword>(db)
        .ok_or(RuleFetchError)?
        .into_iter()
        .map(|keyword| keyword.keyword)
        .collect();

    let share_rule = rule.share_rule.try_into().map_err(|_| RuleFetchError)?;

    Ok(ResponseRule {
        id: rule.id,
        name: rule.name,
        keywords,
        categories,
        share_rule,
    })
}

pub fn get_rules() -> Result<Vec<ResponseRule>, RuleFetchError> {
    let db = get_db();

    let rules = SelectQuery::new().order_by(Rule::name(), Ordering::Ascending).get_all(&db).ok_or(RuleFetchError)?;

    let rules = Result::from_iter(
        rules.into_iter().map(|rule| to_response_rule(rule, &db))
    )?;

    Ok(rules)
}

pub fn get_rule(id: Key) -> Result<ResponseRule, RuleFetchError> {
    let db = get_db();

    let rule = SelectQuery::new().filter(Rule::id().eq(id)).get_first::<Rule>(&db).ok_or(RuleFetchError)?;

    to_response_rule(rule, &db)
} 

pub fn insert_rule(
    id: Option<Key>,
    name: String,
    shared: ShareRule,
    keywords: Vec<String>,
    categories: Vec<Key>,
) -> Result<Key, RuleInsertError> {
    let db = get_db();

    let rule_id = db.insert(Rule {
        id: id.unwrap_or_else(Key::new),
        name,
        share_rule: shared.into(),
    }).ok_or(RuleInsertError)?;

    for keyword in keywords {
        db.insert(RuleKeyword {
            id: Key::new(),
            rule: rule_id.clone(),
            keyword,
        });
    }

    for category in categories {
        db.insert(RuleCategoryLink {
            id: Key::new(),
            rule: rule_id.clone(),
            category,
        });
    }

    Ok(rule_id)
}
