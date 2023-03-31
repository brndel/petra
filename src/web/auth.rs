use base64::{engine::general_purpose, Engine};
use web_server::Request;

pub fn get_auth_name(request: &Request) -> Option<String> {
  let header = request.header("Authorization")?;
  let auth = header.strip_prefix("Basic ")?;
  let bytes = general_purpose::STANDARD.decode(auth).ok()?;
  let auth = String::from_utf8(bytes).ok()?;
  let auth_name = auth.split(':').nth(0)?;

  Some(String::from(auth_name))
}