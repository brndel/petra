use web_server::{HttpCode, Response};

#[derive(Debug)]
pub enum Error {
  Inernal(String),
  Database,
  BadRequest(String),
  Auth,
  NotFound,
}

impl Into<Response> for Error {
  fn into(self) -> Response {
    let mut response = Response::new();
    match self {
      Error::Inernal(_) => {
        response.response_code = HttpCode::_500;
      }
      Error::Database => {
        response.response_code = HttpCode::_500;
      }
      Error::BadRequest(_) => {
        response.response_code = HttpCode::_400;
      }
      Error::Auth => {
        response.response_code = HttpCode::_401;
        response.set_header("WWW-Authenticate", "Basic realm=\"User\"");
      }
      Error::NotFound => {
        response.response_code = HttpCode::_404;
      }
    }

    response
  }
}
