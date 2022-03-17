use hyper::{body::Bytes, Method};

use super::Error;

pub mod message;

#[derive(Debug, Clone)]
pub enum Endpoint {
    Message(String),
}

pub trait ParseBody<T> {
    fn parse_body(&self, body: Bytes) -> Result<T, Error>;
}

impl ParseBody<String> for Endpoint {
    fn parse_body(&self, body: hyper::body::Bytes) -> Result<String, Error> {
        Ok(format!("{:?}", body))
    }
}

impl Endpoint {
    pub fn body(&self) -> hyper::Body {
        match self {
            _ => hyper::Body::empty(),
        }
    }

    pub fn method(&self) -> Method {
        match self {
            _ => Method::GET,
        }
    }

    pub fn path(&self) -> String {
        match self {
            Endpoint::Message(_) => message::path(self),
        }
    }

    pub fn params(&self) -> Vec<(String, String)> {
        match self {
            Endpoint::Message(_) => message::params(self),
        }
    }
}
