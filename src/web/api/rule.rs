use crate::{tables::rule::Rule, Error, Request};

use super::serialize;

pub fn get_rules(request: &Request) -> Result<String, Error> {
  let rules = Rule::get_rules(request.database)?;
  let rules: Vec<_> = rules
    .into_iter()
    .map(|e| e.into_response(request.database))
    .collect();

  serialize(&rules)
}
