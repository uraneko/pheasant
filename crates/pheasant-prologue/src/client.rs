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

impl From<crate::server::Request> for Request {
    fn from(req: crate::server::Request) -> Self {
        let crate::server::Request {
            method,
            path,
            query,
            proto,
            headers,
            body,
        } = req;
        let headers = headers
            .into_iter()
            .map(|h| h.into_bytes())
            .flatten()
            .collect();

        Self {
            method,
            path,
            query,
            proto,
            headers,
            body,
        }
    }
}

#[derive(Debug)]
pub struct Request {
    method: Method,
    path: Path,
    query: Option<Query>,
    proto: Protocol,
    headers: Vec<u8>,
    body: Option<Vec<u8>>,
}

impl Request {
    pub fn new(method: Method, path: Path, proto: Protocol) -> Self {
        Self {
            method,
            path,
            proto,
            query: None,
            headers: Vec::new(),
            body: None,
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

    pub fn headers_mut(&mut self) -> &mut Vec<u8> {
        &mut self.headers
    }

    pub fn body_mut(&mut self) -> Option<&mut Vec<u8>> {
        self.body.as_mut()
    }

    pub fn stream_bytes(&self) -> impl IntoIterator<Item = u8> {
        let q = self
            .query
            .as_ref()
            .map(|q| q.to_bytes())
            .unwrap_or_else(|| Vec::new());

        let b = self
            .body
            .as_ref()
            .map(|b| b.clone())
            .unwrap_or_else(|| Vec::new());

        self.method
            .as_bytes()
            .into_iter()
            .chain(Some(&32))
            .copied()
            .chain(self.path.serialized().into_bytes())
            .chain(q)
            .chain(Some(32))
            .chain(self.proto.as_bytes().into_iter().map(|b| *b))
            .chain(Some(10))
            .chain(self.headers.as_slice().into_iter().map(|b| *b))
            .chain(Some(10))
            .chain(b)
    }

    pub fn clear(&mut self) {
        self.method = Method::Get;
        self.headers.clear();
        if let Some(ref mut body) = self.body {
            body.clear();
        }
    }

    pub fn path_str(&self) -> alloc::string::String {
        self.path.serialized()
    }
}

impl From<Respond> for crate::server::Respond {
    fn from(resp: Respond) -> Self {
        let Respond {
            proto,
            status,
            headers,
            body,
        } = resp;
        let headers = headers
            .into_iter()
            .map(|h| h.into_bytes())
            .flatten()
            .collect();

        Self {
            proto,
            status,
            headers,
            body,
        }
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
    pub fn respond(&mut self) -> Result<Respond, Error> {
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
