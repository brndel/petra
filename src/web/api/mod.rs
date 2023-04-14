mod category;
mod current_user;
mod month_index;
mod payment;
mod rule;
mod user;

use std::{ffi::OsStr, path::Component::Normal};

pub use category::get_categories;
pub use payment::get_payments;
use serde::Serialize;
use web_server::Response;

use crate::{
  request::Method::{Delete, Get, Post},
  Error, Request,
};

use self::{
  category::get_category_groups, current_user::get_current_user, month_index::get_month_index,
  payment::post_payments, rule::get_rules, user::get_users,
};

pub fn handle_api(request: &Request) -> Option<Result<Response, Error>> {
  let mut components = request.path.components();

  if components.next() == Some(Normal(OsStr::new("api"))) {
    if let Some(method) = components.next().and_then(|e| e.as_os_str().to_str()) {
      return Some(handle_methods(method, request));
    }
  }

  None
}

fn handle_methods(method: &str, request: &Request) -> Result<Response, Error> {
  let mut response = Response::new();

  let resp = match request.method {
    Get => match method {
      // User data
      "current_user" => get_current_user(request),
      "month_index" => get_month_index(request),
      "payment" => get_payments(request),
      // Global data
      "user" => get_users(request),
      "category_group" => get_category_groups(request),
      "category" => get_categories(request),
      "rule" => get_rules(request),
      _ => Err(Error::NotFound),
    }?,
    Post => match method {
      "payment" => post_payments(request),
      _ => Err(Error::NotFound),
    }?,
    Delete => match method {
      _ => Err(Error::NotFound),
    }?,
  };

  response.set_body(resp.as_str());
  response.set_header("Content-Type", "application/json");

  Ok(response)
}

fn serialize<T: Serialize>(data: &T) -> Result<String, Error> {
  serde_json::to_string(data).map_err(|_| Error::Inernal("could not serialize result".to_string()))
}
