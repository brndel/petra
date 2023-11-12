use actix_web::dev::ServiceRequest;
use actix_web::{Error, HttpMessage};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::AuthenticationError;

use crate::api::user::get_user_by_name;
use actix_web_httpauth::headers::www_authenticate::basic::Basic;
use leptos::ServerFnError;
use mensula::Key;

pub async fn authenticate_user(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    fn inner(credentials: &BasicAuth) -> Option<Key> {
        let password = credentials.password()?;
        get_authenticated_user(credentials.user_id(), password)
    }

    match inner(&credentials) {
        Some(user_key) => {
            req.extensions_mut().insert(user_key);
            Ok(req)
        }
        None => Err((AuthenticationError::new(Basic::default()).into(), req)),
    }
}

fn get_authenticated_user(name: &str, password: &str) -> Option<Key> {
    let user = get_user_by_name(name.to_owned())?;

    if user.authenticate(password) {
        Some(user.id)
    } else {
        None
    }
}

pub async fn get_user() -> Result<Key, ServerFnError> {
    use actix_web::web;
    use leptos_actix::extract;

    extract(|user: web::ReqData<Key>| async move { (*user).clone() }).await
}

pub fn get_actix_user<R: HttpMessage>(req: &R) -> Option<Key> {
    req.extensions().get().cloned()
}