use chrono::{Duration, Local};
use data::Database;
use serde::{Deserialize, Serialize};

use crate::{tables::tink_token::TinkToken, web::api::{serialize, tink::tink_secret::{get_tink_client_id, get_tink_client_secret}}, Error, Request};

use super::{
  get_token,
};

pub fn get_tink_token_callback(request: &Request) -> Result<String, Error> {
  println!("TINK TOKEN CALLBACK");
  let code = request
    .params
    .get("code")
    .ok_or(Error::BadRequest("Expected 'code' parameter".to_string()))?;
  // let credentials = request.params.get("credentials");

  let tink_data = request_tink_token(code).map_err(|e| {
    println!("[tink] error: '{}'", e);
    Error::Internal
  })?;

  println!("Received Tink data {:?}", tink_data);

  let res = tink_data.insert(request.database, request.user_id);

  println!("Inserted TinkToken {:?}", res);

  Err(Error::Redirect("/add"))
}

#[derive(Deserialize, Debug)]
struct TinkAuthData {
  access_token: String,
  id_hint: String,
  expires_in: i64,
  refresh_token: String,
  scope: String,
  token_type: String,
}

impl TinkAuthData {
  fn insert(self, database: &Database, user_id: i64) -> Option<i64> {
    let now = Local::now();
    let expires = now.checked_add_signed(Duration::seconds(self.expires_in))?;
    let expires_timestamp = expires.to_rfc3339();

    database.insert(TinkToken {
      user_id,
      token: self.access_token,
      expires_timestamp,
    })
  }
}

fn request_tink_token(code: &str) -> Result<TinkAuthData, minreq::Error> {
  // let body = json!({
  //   "code": code,
  //   "client_id": CLIENT_ID,
  //   "client_secret": CLIENT_SECRET,
  //   "grant_type": "authorization_code"
  // });

  let body = format!(
    "code={}&client_id={}&client_secret={}&grant_type={}",
    code, get_tink_client_id(), get_tink_client_secret(), "authorization_code"
  );

  let request = minreq::post("https://api.tink.com/api/v1/oauth/token")
    .with_body(body)
    .with_header("Content-Type", "application/x-www-form-urlencoded");

  let response = request.send()?;
  // println!("Received Response {:?}", response);
  println!("Received response with body '{:?}'", response.as_str());

  let data: TinkAuthData = response.json()?;

  Ok(data)
}

#[derive(Serialize)]
struct TinkTokenResponse {
  expires_timestamp: String,
}

pub fn get_tink_token(request: &Request) -> Result<String, Error> {
  let token = get_token(request).map(|t| TinkTokenResponse {
    expires_timestamp: t.expires_timestamp,
  });

  serialize(&token)
}
