use web_server::{Response, Request, HttpCode};

use crate::web::get_auth_name;



pub fn get_index(request: Request, mut response: Response) -> Response {
  println!("index get");
  if let Some(auth) = get_auth_name(&request) {
    println!("auth: '{}'", auth);
    return "Hello world!".into();
  } else {
    println!("no auth");
    response.response_code = HttpCode::_401;
    response.set_header(
      "WWW-Authenticate",
      "Basic realm=\"Hellow\"",
    );
    return response;
  }
}