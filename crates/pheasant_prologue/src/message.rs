use crate::{Method, Protocol};
use alloc::vec::Vec;
use pheasant_uri::Resource;

pub mod http11;

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
    pub fn is_eol(&self) -> bool {
        [Self::CRLF, Self::LFCR, Self::LF].contains(&self)
    }
}
