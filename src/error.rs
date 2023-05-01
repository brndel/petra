use web_server::{HttpCode, Response};

#[derive(Debug)]
pub enum Error {
  Internal,
  Database,
  BadRequest(String),
  Auth,
  Redirect(&'static str),
  NotFound,
}

impl Into<Response> for Error {
  fn into(self) -> Response {
    let mut response = Response::new();
    match self {
      Error::Internal => {
        response.response_code = HttpCode::_500;
      }
      Error::Database => {
        response.response_code = HttpCode::_500;
      }
      Error::BadRequest(msg) => {
        response.response_code = HttpCode::_400;
        response.set_body(&msg);
      }
      Error::Auth => {
        response.response_code = HttpCode::_401;
        response.set_header("WWW-Authenticate", "Basic realm=\"User\"");
      }
      Error::Redirect(location) => {
        response.response_code = HttpCode::_301;
        response.set_header("Location", &location);
      }
      Error::NotFound => {
        response.response_code = HttpCode::_404;
      }
    }

    response
  }
}
