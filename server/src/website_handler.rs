use super::server::Handler;
use crate::http::{Response, Request, StatusCode, Method};
use std::fs;
pub struct WebsiteHandler{
    public_path: String
}

impl WebsiteHandler{
    pub fn new(public_path: String) -> Self {
        Self {public_path}
    }

    fn read_file(&self, file_path: &str) -> Option<String>{
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(full_path) => {
                if full_path.starts_with(&self.public_path) {
                    fs::read_to_string(full_path).ok()
                } else {
                    println!("Someone is attempting to hack the mainframe!! Directory Traversal Attack Attempted Path: {}", file_path);
                    None
                }
            },
            Err(_) => None,
        }

    }
}

impl Handler for WebsiteHandler{
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => {
                    match self.read_file(path) {
                        Some(body) => Response::new(StatusCode::Ok, Some(body)),
                        None => Response::new(StatusCode::NotFound, None)
                    }
                }
            }
            _=> Response::new(StatusCode::BadRequest, None)
        }
    }
}