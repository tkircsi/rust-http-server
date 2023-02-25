use super::http::{Method, ParseError, Request, Response, SatusCode};
use super::server::Handler;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!(
                        "directory traversal attack detected: {}",
                        path.to_str().unwrap_or("")
                    );
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(SatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(SatusCode::Ok, self.read_file("hello.html")),
                "/bad" => Response::new(SatusCode::BadRequest, None),
                path => match self.read_file(path) {
                    Some(content) => Response::new(SatusCode::Ok, Some(content)),
                    None => Response::new(SatusCode::NotFound, None),
                },
            },
            _ => Response::new(SatusCode::NotFound, None),
        }
    }

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        Response::new(SatusCode::BadRequest, Some(format!("Error: {}", e)))
    }
}
