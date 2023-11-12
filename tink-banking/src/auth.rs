use chrono::{Duration, Local, DateTime, FixedOffset};
use serde::Deserialize;

use crate::config::get_config;

static AUTH_URL: &str = "https://api.tink.com/api/v1/oauth/token";

#[derive(Debug)]
pub struct AuthToken {
    pub token: String,
    pub expires_timestamp: DateTime<FixedOffset>,
}

pub fn get_auth_token(auth_code: &str) -> Option<AuthToken> {
    let config = get_config();

    let body = format!(
        "code={}&client_id={}&client_secret={}&grant_type={}",
        auth_code,
        config.id,
        config.secret,
        "authorization_code"
    );

    let request = minreq::post(AUTH_URL)
        .with_body(body)
        .with_header("Content-Type", "application/x-www-form-urlencoded");

    let response = request.send().ok()?;

    #[derive(Deserialize, Debug)]
    struct Response {
        access_token: String,
        expires_in: i64,
    }

    let response: Response = response.json().ok()?;

    let expires = Local::now().checked_add_signed(Duration::seconds(response.expires_in))?;

    Some(AuthToken {
        token: response.access_token,
        expires_timestamp: expires.fixed_offset(),
    })
}
