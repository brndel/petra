use crate::{tables::user::User, Error, Request};

use super::serialize;

pub fn get_users(request: &Request) -> Result<String, Error> {
  let users = request
    .database
    .get_all::<User>();

  serialize(&users)
}
