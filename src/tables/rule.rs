use data::{query::SelectQuery, Database, PrimKey, Table};
use serde::Serialize;

use crate::Error;

use super::category::Category;

const NOT_SHARED: i64 = 0;
const SHARED: i64 = 1;
const CHOOSE: i64 = 2;

#[derive(Table)]
pub struct Rule {
  #[primary]
  pub id: PrimKey,
  pub name: String,
  pub shared: i64,
}

#[derive(Serialize)]
pub struct RuleResponse {
  id: i64,
  name: String,
  shared: Option<bool>,
  categories: Vec<i64>,
  keywords: Vec<String>,
}

impl Rule {
  pub fn get_rules(database: &Database) -> Result<Vec<Rule>, Error> {
    database.get_all::<Self>().ok_or(Error::Database)
  }

  pub fn into_response(self, database: &Database) -> RuleResponse {
    let shared = self.is_shared();
    let categories = self.get_categories(database);
    let keywords = self.get_keywords(database);
    RuleResponse {
      id: self.id,
      name: self.name,
      shared,
      categories,
      keywords,
    }
  }

  fn is_shared(&self) -> Option<bool> {
    match self.shared {
      NOT_SHARED => Some(false),
      SHARED => Some(true),
      CHOOSE => None,
      _ => None,
    }
  }

  pub fn get_share_num(shared: &Option<bool>) -> i64 {
    match shared {
      Some(true) => SHARED,
      Some(false) => NOT_SHARED,
      None => CHOOSE,
    }
  }

  fn get_categories(&self, database: &Database) -> Vec<i64> {
    database
      .get_linked::<Category, Rule, RuleCategoryLink>(self.get_primary())
      .unwrap_or_default()
  }

  fn get_keywords(&self, database: &Database) -> Vec<String> {
    let keywords: Vec<RuleKeyword> = SelectQuery::new()
      .filter(RuleKeyword::rule_id().eq(self.get_primary()))
      .get_all(database)
      .unwrap_or_default();

    keywords.into_iter().map(|k| k.keyword).collect()
  }
}

// Links

#[derive(Table)]
pub struct RuleCategoryLink {
  #[primary]
  pub id: i64,
  #[foreign_link(Rule)]
  pub rule_id: i64,
  #[foreign_link(Category)]
  pub category_id: i64,
}

#[derive(Table)]
pub struct RuleKeyword {
  #[primary]
  pub id: i64,
  pub keyword: String,
  #[foreign(Rule)]
  pub rule_id: i64,
}
