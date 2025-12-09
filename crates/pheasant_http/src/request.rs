use crate::{Method, Protocol};

use alloc::string::String;
use alloc::vec::Vec;
use pheasant_uri::{Path, Query, Resource};

pub mod http11;

#[derive(Debug)]
pub struct Request {
    method: Method,
    path: Path,
    query: Option<Query>,
    proto: Protocol,
    headers: Vec<Header>,
    body: Option<Vec<u8>>,
}

impl Request {
    pub fn method(&self) -> Method {
        self.method
    }

    pub fn proto(&self) -> Protocol {
        self.proto
    }

    pub fn path(&self) -> String {
        self.path.serialized()
    }

    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }

    pub fn headers(&self) -> &[Header] {
        &self.headers
    }

    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_ref().map(|b| b.as_slice())
    }
}

#[derive(Debug, PartialEq)]
pub struct Header {
    field: Vec<u8>,
    value: Vec<u8>,
}

impl Header {
    pub fn new(field: Vec<u8>, value: Vec<u8>) -> Self {
        Self { field, value }
    }

    pub fn field(&self) -> &[u8] {
        &self.field
    }

    pub fn value(&self) -> &[u8] {
        &self.value
    }
}

pub fn contains_header(headers: &[Header], header: &[u8]) -> bool {
    headers.iter().any(|Header { field, .. }| field == header)
}

pub fn header_value<'a>(headers: &'a [Header], header: &[u8]) -> Option<&'a [u8]> {
    headers
        .iter()
        .find_map(|Header { field, value }| (field == header).then(|| value.as_slice()))
}

// TODO add a feature in this crate `raw-tokens`
// when this is on, all tokens are stored as Vec<u8> (including method, protocol...)
#[derive(Debug, PartialEq)]
pub enum Token {
    Method(Method),
    Path(Resource),
    Protocol(Protocol),
    LF,
    CRLF,
    // there is really no difference between \n\r and \r\n so we accept both
    LFCR,
    Field(Vec<u8>),
    Value(Vec<u8>),
    Body(Vec<u8>),
}

impl Token {
    fn is_eol(&self) -> bool {
        [Self::CRLF, Self::LFCR, Self::LF].contains(&self)
    }
}
