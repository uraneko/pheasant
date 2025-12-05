use crate::{Headers, Method, Protocol};

use alloc::vec::Vec;
use pheasant_uri::{Path, Query, Resource};

pub mod http11;

pub struct Request {
    method: Method,
    path: Path,
    query: Option<Query>,
    proto: Protocol,
    headers: Headers,
    body: Option<Vec<u8>>,
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
