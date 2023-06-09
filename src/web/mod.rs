mod api;
mod file;

pub use api::tink::tink_secret;

use std::path::Path;

use base64::{engine::general_purpose, Engine};
use data::{query::SelectQuery, Database};
use sha256::digest;
use web_server::Response;

use crate::{request::Method, tables::user::User, Error, Request};

use self::{api::handle_api, file::handle_file};

pub fn convert_to_username(name: &str) -> String {
  name.to_lowercase().replace(' ', "_")
}

pub fn get_auth_hash(username: &str, password: &str) -> String {
  digest(format!("{}:{}", username, password).as_str())
}

pub fn handle(request: web_server::Request, database: &Database) -> Result<Response, Error> {
  let mut path_str = request.get_path();
  path_str = path_str
    .strip_prefix('/')
    .and_then(|s| Some(s.to_owned()))
    .unwrap_or(path_str);
  path_str = path_str
    .split('?')
    .nth(0)
    .and_then(|s| Some(s.to_owned()))
    .unwrap_or(path_str);

  let mut path = Path::new(&path_str);

  if path.as_os_str().is_empty() {
    path = Path::new("index.html");
  }

  let request = create_request(&request, database, path)?;
  let response = handle_request(&request)?;

  Ok(response)
}

pub fn create_request<'a>(
  request: &web_server::Request,
  database: &'a Database,
  path: &'a Path,
) -> Result<Request<'a>, Error> {
  fn get_user_id(request: &web_server::Request, database: &Database) -> Option<i64> {
    let header = request.header("Authorization")?;
    let auth = header.strip_prefix("Basic ")?;
    let bytes = general_purpose::STANDARD.decode(auth).ok()?;
    let auth = String::from_utf8(bytes).ok()?;
    let parts: Vec<&str> = auth.split(':').collect();

    let auth_name = convert_to_username(parts.get(0)?);
    let auth_password = parts.get(1)?;

    let auth_hash = get_auth_hash(&auth_name, &auth_password);

    // println!("hash: '{}'", auth_hash);
    
    SelectQuery::new().filter(User::name().eq(auth_name).and(User::auth_hash().eq(auth_hash))).get_first(database)
  }

  let user_id = get_user_id(request, database).ok_or(Error::Auth)?;

  let method = Method::try_from(request.get_method())
    .map_err(|_| Error::BadRequest("Invalid method".to_string()))?;

  let params = request.query.to_owned();

  Ok(Request {
    user_id,
    database,
    path,
    method,
    params,
    body: request.get_body(),
  })
}

fn handle_request(request: &Request) -> Result<Response, Error> {
  // println!("{request:?}");
  let response = handle_api(request).or_else(|| handle_file(request));

  response.unwrap_or_else(|| Err(Error::NotFound))
}
