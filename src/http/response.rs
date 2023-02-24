use super::status_code::SatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: SatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: SatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
}
