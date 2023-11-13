use leptos::{server, ServerFnError};
use chrono::{DateTime, FixedOffset};
use mensula_key::Key;
use serde::{Serialize, Deserialize};

use crate::util::month::MonthDate;

pub use super::data::*;

#[cfg(feature = "ssr")]
use super::server;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TinkPaymentData {
    pub name: String,
    pub amount: i64,
    pub timestamp: DateTime<FixedOffset>,
}

#[server]
pub async fn tink_get_token_timeout() -> Result<Option<DateTime<FixedOffset>>, ServerFnError> {
    let user = crate::auth::get_user().await?;
    Ok(server::get_token(user).map(|token| token.expires_timestamp))
}

#[server]
pub async fn tink_get_payments(month: MonthDate) -> Result<Vec<TinkPayment>, ServerFnError> {
    let user = crate::auth::get_user().await?;

    server::get_payments(user, month)
        .ok_or_else(|| ServerFnError::ServerError("Could not get payments".to_string()))
}

#[server]
pub async fn tink_get_payment_data(id: Key) -> Result<TinkPaymentData, ServerFnError> {
    // let user = crate::auth::get_user().await?;

    server::get_payment_data(id, None).ok_or_else(||ServerFnError::ServerError("Unkown tink payment id".to_string()))
}

#[server]
pub async fn tink_get_url() -> Result<String, ServerFnError> {
    Ok(server::get_tink_url())
}

#[cfg(feature = "ssr")]
use actix_web::{HttpRequest, HttpResponse};

#[cfg(feature = "ssr")]
#[actix_web::get("api/tink/callback")]
async fn token_callback(req: HttpRequest) -> HttpResponse {
    use actix_web::web::Query;

    use crate::auth::get_actix_user;

    #[derive(Deserialize)]
    struct Params {
        code: String,
        // #[serde(rename("credentialsId"))]
        // credentials_id: String,
    }

    match Query::<Params>::from_query(req.query_string()) {
        Ok(params) => {
            let user = match get_actix_user(&req) {
                Some(user) => user,
                None => {
                    return HttpResponse::BadRequest()
                        .body("missing authentication")
                        .into()
                }
            };

            if let Some(_) = server::create_token(user, &params.code) {
                let mut resp = HttpResponse::PermanentRedirect();
                resp.append_header(("Location", "/add"));
                resp.into()
            } else {
                HttpResponse::InternalServerError()
                    .body("could not create token")
                    .into()
            }
        }
        Err(err) => HttpResponse::BadRequest().body(err.to_string()).into(),
    }
}
