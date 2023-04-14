use std::{path::Path, collections::HashMap};

use data::Database;
use web_server::HttpMethod;

#[derive(Debug)]
pub struct Request<'a> {
  pub user_id: i64,
  pub database: &'a Database,
  pub path: &'a Path,
  pub method: Method,
  pub params: HashMap<String, String>,
  pub body: String,
}

#[derive(Debug, PartialEq)]
pub enum Method {
  Get,
  Post,
  Delete,
}

impl TryFrom<HttpMethod> for Method {
  type Error = ();

  fn try_from(value: HttpMethod) -> Result<Self, Self::Error> {
    let method = match value {
      HttpMethod::GET => Self::Get,
      HttpMethod::POST => Self::Post,
      HttpMethod::DELETE => Self::Delete,
      _ => return Err(()),
    };
    Ok(method)
  }
}
