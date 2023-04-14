use data::{query::SelectQuery, Database, Table};
use serde::Serialize;

use crate::Error;

use super::{category::Category};

pub const SHARED_NONE: i64 = 0;
pub const SHARED_ALL: i64 = 1;
pub const SHARED_MAYBE: i64 = 2;

#[derive(Table)]
pub struct Rule {
  #[primary]
  pub id: i64,
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
    Ok(database.get_all::<Self>())
  }

  pub fn into_response(self, database: &Database) -> RuleResponse {
    RuleResponse {
      id: self.id,
      name: self.name,
      shared: Self::is_shared(self.shared),
      categories: Self::get_categories(self.id, database),
      keywords: Self::get_keywords(self.id, database),
    }
  }

  fn is_shared(shared: i64) -> Option<bool> {
    match shared {
      SHARED_NONE => Some(false),
      SHARED_ALL => Some(true),
      SHARED_MAYBE => None,
      _ => None,
    }
  }

  fn get_categories(id: i64, database: &Database) -> Vec<i64> {
    database.get_linked::<Category, Rule, RuleCategoryLink>(id)
  }

  fn get_keywords(id: i64, database: &Database) -> Vec<String> {
    let keywords: Vec<RuleKeyword> = SelectQuery::new()
      .filter(RuleKeyword::rule_id().eq(id))
      .get_all(database);

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
