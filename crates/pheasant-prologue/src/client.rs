use crate::{
    Header, Method, Protocol, Status,
    message::{
        Token,
        http11::{Error, Lex, build_headers, content_length},
    },
};
use alloc::vec::Vec;
use embedded_io::{Read, Write};
use pheasant_uri::{Path, Query};

pub struct Request<H: Read + Write, B: Read + Write> {
    method: Method,
    path: Path,
    query: Option<Query>,
    proto: Protocol,
    headers: H,
    body: Option<B>,
}

impl<H: Read + Write, B: Read + Write> Request<H, B> {
    pub fn new(method: Method, path: Path, proto: Protocol, headers: H) -> Self {
        Self {
            method,
            path,
            proto,
            query: None,
            headers,
            body: None,
        }
    }

    pub fn with_body(method: Method, path: Path, proto: Protocol, headers: H, b: B) -> Self {
        Self {
            method,
            path,
            proto,
            query: None,
            headers,
            body: Some(b),
        }
    }

    pub fn method<M>(mut self, method: M) -> Result<Self, M::Error>
    where
        M: TryInto<Method>,
    {
        self.method = method.try_into()?;

        Ok(self)
    }

    pub fn proto<P>(mut self, proto: P) -> Result<Self, P::Error>
    where
        P: TryInto<Protocol>,
    {
        self.proto = proto.try_into()?;

        Ok(self)
    }

    pub fn path<P>(mut self, path: P) -> Result<Self, P::Error>
    where
        P: TryInto<Path>,
    {
        self.path = path.try_into()?;

        Ok(self)
    }

    pub fn query_mut(&mut self) -> Option<&mut Query> {
        self.query.as_mut()
    }

    pub fn headers_mut(&mut self) -> &mut H {
        &mut self.headers
    }

    pub fn body_mut(&mut self) -> Option<&mut B> {
        self.body.as_mut()
    }
}

#[derive(Debug)]
pub struct Respond {
    proto: Protocol,
    status: Status,
    headers: Vec<Header>,
    body: Vec<u8>,
}

impl<'a> Lex<'a> {
    pub fn response(&mut self) -> Result<Respond, Error> {
        let proto = self.resp_proto()?;
        let status = self.status()?;
        let headers = self.headers()?;
        let len = content_length(&headers);
        let len = match len {
            Err(Error::ContentLengthNotFound) => self.len() - self.cursor,
            Ok(len) => len,
            Err(err) => return Err(err),
        };
        let body = match self.body(len)? {
            Some(Token::Body(body)) => body,
            Some(_) => return Err(Error::UndesirableToken),
            None => Vec::new(),
        };
        let headers = build_headers(headers)?;

        Ok(Respond {
            proto,
            status,
            headers,
            body,
        })
    }
}

impl Respond {
    pub fn proto(&self) -> Protocol {
        self.proto
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn headers(&self) -> &[Header] {
        &self.headers
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }
}
