use serde::Deserialize;

use crate::{tables::user::{User, UserInsert}, Error, Request, web::get_auth_hash};

use super::serialize;

pub fn get_users(request: &Request) -> Result<String, Error> {
  let users = request
    .database
    .get_all::<User>();

  serialize(&users)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AddUserQuery {
  display_name: String,
  name: String,
  password: String,
}

pub fn add_user(request: &Request) -> Result<String, Error> {
  println!("adding user with data '{}'", request.body);
  let query: AddUserQuery = serde_json::from_str(&request.body).map_err(|_| Error::BadRequest("could not deserialize request".to_string()))?;

  println!("{:?}", query);

  let auth_hash = get_auth_hash(&query.name, &query.password);

  request.database.insert(UserInsert {
    display_name: query.display_name,
    name: query.name,
    auth_hash,
  }).ok_or_else(|| Error::Database)?;

  println!("user added");

  Ok("null".into())
}