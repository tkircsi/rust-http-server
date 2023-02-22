use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        /* Solution 1:
            match str::from_utf8(buf) {
            Ok(req) => {}
            Err(_) => return Err(ParseError::InvalidEncoding),
        }
        */

        /* Solution 2:
        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
            Ok(req) => {}
            Err(e) => Err(e),
        }
        */

        // Solution 3:
        // let req = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        // Solution 4:
        // Implemented From trait for ParseError for fconversion.
        let req = str::from_utf8(buf)?;

        /* Solution 1:
        match get_next_word(req) {
            Some((method, req)) => {}
            None => return Err(ParseError::InvalidRequest),
        }
        */

        // Solution 2
        let (method, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (path, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        Ok(Request {})
    }
}

fn get_next_word(string: &str) -> Option<(&str, &str)> {
    for (i, c) in string.char_indices() {
        if c == ' ' || c == '\r' {
            return Some((&string[..i], &string[i + 1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {}
