use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
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
        let (mut path, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        /* let p: Vec<&str> = path.split('?').collect();
        let (path, query_string) = match p.len() {
            2 => {
                if p[1].len() == 0 {
                    (p[0], None)
                }else{
                    (p[0], Some(p[1].to_string()))
                }
            },
            1 => (p[0], None),
            _ => ("/", None),
        }; */

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            method,
            path,
            query_string,
        })
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
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Error for ParseError {}
