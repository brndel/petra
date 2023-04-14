use std::{path::{Path, PathBuf}, fs::File, io::Read};

use web_server::Response;

use crate::{Error, request::Method, Request};

const SOURCE_PATH: &str = "web/build";

pub fn handle_file(request: &Request) -> Option<Result<Response, Error>> {
  if request.method != Method::Get {
    return None
  }

  let extension = request.path.extension()?.to_str()?;

  let body = read_file(&request.path);
  match body {
    Err(error) => Some(Err(error)),
    Ok(body) => {
      let mut response = Response::new();

      response.set_body(&body);
      response.set_header("Content-Type", content_type(extension));

      Some(Ok(response))
    }
  }
}

fn read_file<P: AsRef<Path>>(path: &P) -> Result<String, Error> {
  let path = PathBuf::from(SOURCE_PATH).join(path);
  let mut content = File::open(path).map_err(|_| Error::NotFound)?;
  let mut buf = String::new();
  content
    .read_to_string(&mut buf)
    .map_err(|_| Error::NotFound)?;
  Ok(buf)
}

fn content_type(extension: &str) -> &str {
  match extension {
    "html" => "text/html",
    "css" => "text/css",
    "js" => "application/javascript",
    "svg" => "svg",
    _ => "unknown",
  }
}
