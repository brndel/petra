use mensula::query::{Ordering, SelectQuery};
use mensula::{Database, Table};
use mensula_key::Key;

use crate::api::category::server::Category;
use crate::db::get_db;

use super::api::{Rule as ResponseRule, RuleFetchError, RuleInsertError, ShareRule};

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
    let categories = SelectQuery::<Category>::link::<Rule, RuleCategoryLink>(rule.id.clone())
        .order_by(Category::name(), Ordering::Ascending)
        .get_all(&db)
        .ok_or(RuleFetchError)?;

    let keywords = SelectQuery::new()
        .filter(RuleKeyword::rule().eq(rule.id.clone()))
        .order_by(RuleKeyword::keyword(), Ordering::Ascending)
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

    let rules = SelectQuery::new()
        .order_by(Rule::name(), Ordering::Ascending)
        .get_all(&db)
        .ok_or(RuleFetchError)?;

    let rules = Result::from_iter(rules.into_iter().map(|rule| to_response_rule(rule, &db)))?;

    Ok(rules)
}

pub fn get_rule(id: Key) -> Result<ResponseRule, RuleFetchError> {
    let db = get_db();

    let rule = SelectQuery::new()
        .filter(Rule::id().eq(id))
        .get_first::<Rule>(&db)
        .ok_or(RuleFetchError)?;

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

    let rule_id = db
        .insert(Rule {
            id: id.unwrap_or_else(Key::new),
            name,
            share_rule: shared.into(),
        })
        .ok_or(RuleInsertError)?;

    if let Some(keywords) = SelectQuery::new()
        .filter(RuleKeyword::rule().eq(rule_id.clone()))
        .get_all::<Key>(&db)
    {
        for keyword in keywords {
            let _ = db.delete::<RuleKeyword>(keyword);
        }
    }

    for keyword in keywords {
        let keyword = clean_keyword(keyword);
        db.insert(RuleKeyword {
            id: Key::new(),
            rule: rule_id.clone(),
            keyword,
        });
    }

    if let Some(cateories) = SelectQuery::new()
        .filter(RuleCategoryLink::rule().eq(rule_id.clone()))
        .get_all::<Key>(&db)
    {
        for category in cateories {
            let _ = db.delete::<RuleCategoryLink>(category);
        }
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

fn clean_keyword(keyword: String) -> String {
    keyword.to_lowercase()
}

pub fn delete_rule(id: Key) -> bool {
    get_db().delete::<Rule>(id).is_ok()
}
