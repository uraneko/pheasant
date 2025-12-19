use crate::{Header, Method, Protocol};

use alloc::string::String;
use alloc::vec::Vec;
use pheasant_uri::{Path, Query, Resource};

pub mod http11;

#[derive(Debug, Clone)]
pub struct Request {
    method: Method,
    path: Path,
    query: Option<Query>,
    proto: Protocol,
    headers: Vec<Header>,
    body: Option<Vec<u8>>,
}

// read/get methods
impl Request {
    pub fn method(&self) -> Method {
        self.method
    }

    pub fn proto(&self) -> Protocol {
        self.proto
    }

    pub fn path(&self) -> &[String] {
        &self.path.segments()
    }

    pub fn path_str(&self) -> String {
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

impl Request {
    /// makes a header value all in lowercase if it exists in request headers
    /// useful for using any service that only works when a request header value matches with one that you provided to the service
    /// and you're not sure about the request header value's case (upper/lower)
    ///
    /// # example
    /// client sends this header: 'access-control-request-headers: Range, Content-type'
    /// the server has this cors configuration: 'access-control-allow-headers: range, content-type'
    /// the headers are allowed, but the Cors service would not understand that
    /// since range != Range && Content-type != content-type
    /// so we lowercase the request header value first
    /// ```
    /// req.lowercase_header_value(b"access-control-request-headers");
    /// ```
    pub fn lowercase_header_value(&mut self, field: &[u8]) {
        let Some(value) = self
            .headers
            .iter_mut()
            .find_map(|h| (h.field_ref() == field).then(|| h.value_mut()))
        else {
            return;
        };

        value.make_ascii_lowercase();
    }

    /// same as the lowercase_header_value method but does many headers' values at once
    pub fn lowercase_header_values(&mut self, fields: &[&[u8]]) {
        self.headers
            .iter_mut()
            .filter(|h| fields.contains(&h.field_ref()))
            .for_each(|h| h.value_mut().make_ascii_lowercase())
    }
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
