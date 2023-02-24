use super::status_code::SatusCode;
use std::{
    //fmt::{Display, Formatter, Result as FmtResult},
    io::{Result as IoResult, Write},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Response {
    status_code: SatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: SatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body,
        )
    }
}

/* impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body,
        )
    }
} */
