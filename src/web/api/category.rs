use crate::{
  tables::category::{Category, CategoryGroup},
  Error, Request,
};

use super::serialize;

pub fn get_categories(request: &Request) -> Result<String, Error> {
  let categories = request.database.get_all::<Category>();

  serialize(&categories)
}

pub fn get_category_groups(request: &Request) -> Result<String, Error> {
  let groups = request.database.get_all::<CategoryGroup>();

  serialize(&groups)
}
