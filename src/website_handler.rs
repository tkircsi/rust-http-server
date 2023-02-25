use super::http::{Method, ParseError, Request, Response, SatusCode};
use super::server::Handler;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(SatusCode::Ok, Some("<h1Welcome!</h1>".to_string())),
                "/hello" => Response::new(SatusCode::Ok, Some("<h1>Hello!</h1>".to_string())),
                "/bad" => Response::new(SatusCode::BadRequest, None),
                _ => Response::new(SatusCode::NotFound, None),
            },
            _ => Response::new(SatusCode::NotFound, None),
        }
    }

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        Response::new(SatusCode::BadRequest, None)
    }
}
