use super::http::{ParseError, Request, Response, SatusCode};
use super::server::Handler;

pub struct WebsiteHandler {}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(
            SatusCode::Ok,
            Some("<h1>Website Handler test</h1>".to_string()),
        )
    }

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        Response::new(SatusCode::BadRequest, None)
    }
}
