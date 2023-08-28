use chrono::{Duration, Local, NaiveDateTime};
use serde::Deserialize;

use crate::secret::get_config;

static AUTH_URL: &str = "https://api.tink.com/api/v1/oauth/token";

pub struct AuthToken {
    pub token: String,
    pub expires_timestamp: NaiveDateTime,
}

pub fn get_auth_token(auth_code: &str) -> Option<AuthToken> {
    let config = get_config();

    let request = minreq::post(AUTH_URL)
        .with_param("code", auth_code)
        .with_param("client_id", config.id)
        .with_param("client_secret", config.secret)
        .with_param("grant_type", "autorization_code")
        .with_header("Content-Type", "application/x-www-form-urlencoded");

    let response = request.send().ok()?;

    #[derive(Deserialize)]
    struct Response {
        access_token: String,
        expires_in: i64,
    }

    let response: Response = response.json().ok()?;

    let expires = Local::now().checked_add_signed(Duration::seconds(response.expires_in))?;

    Some(AuthToken {
        token: response.access_token,
        expires_timestamp: expires.naive_utc(),
    })
}
