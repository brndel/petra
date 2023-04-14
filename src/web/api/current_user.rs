use serde_json::json;

use crate::{Request, Error};

pub fn get_current_user(request: &Request) -> Result<String, Error> {
  let value = json!({"user": request.user_id});
  Ok(value.to_string())
}