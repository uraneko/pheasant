//! this crate defines the request api
//!
//! ### APIs
//! - parse (from bytes into request)
//! - builder
//! - headers (read)

use hashbrown::{HashMap, HashSet};
use pheasant_core::{ErrorStatus, Method, Protocol, err_stt};
use pheasant_headers::{Cookie, Headers, RequestCors};
use pheasant_uri::{Query, Resource, Route};

pub mod builder;
pub mod headers;
pub mod http11;

use builder::Builder;
use http11::lex::Token;

#[derive(Debug)]
enum Error {
    TokenMismatch,
}

#[derive(PartialEq, Eq)]
pub struct Request {
    headers: Headers,
    proto: Protocol,
    method: Method,
    route: Route,
    query: Option<Query>,
    cors: Option<RequestCors>,
    cookies: Option<HashSet<Cookie>>,
    body: Option<Vec<u8>>,
}

impl core::fmt::Debug for Request {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Request {{\n   {} {} {}\n   headers: {:#?},\n   cookies: {:?},\n   cors: {:?},\n   body: {:?}\n}}",
            self.method,
            {
                if let Some(q) = &self.query {
                    format!("{}{}", self.route.to_string(), q.sequence())
                } else {
                    self.route.to_string()
                }
            },
            self.proto,
            self.headers,
            self.cors,
            self.cookies,
            self.body
                .as_ref()
                .map(|b| if b.len() > 19 {
                    format!("{:?}...", &b[..20])
                } else {
                    format!("{:?}", &b)
                })
                .unwrap_or_default()
        )
    }
}

impl Request {
    pub fn builder(method: Method, resource: Resource, proto: Protocol) -> Builder {
        Builder {
            method,
            resource,
            proto,
            headers: Headers::default(),
            body: None,
        }
    }

    pub fn parse(mut tokens: Vec<Token>) -> Result<Self, ErrorStatus> {
        let body = match tokens.last().map(|t| t.is_body()) {
            Some(true) => tokens.pop().map(|t| t.into_vec()).flatten(),
            _ => None,
        };

        let mut iter = tokens.into_iter();
        let [
            Some(Token::Method(method)),
            Some(Token::Uri(resource)),
            Some(Token::Proto(proto)),
        ] = [iter.next(), iter.next(), iter.next()]
        else {
            return err_stt!(?BadRequest);
        };

        let mut builder = Request::builder(method, resource, proto);
        builder.body(body);

        while let Some(Token::Header(header)) = iter.next() {
            let Some(Token::Field(field)) = iter.next() else {
                return Err(err_stt!(BadRequest));
                // "expected field after header token"
            };
            builder
                .header(header, field)
                .map_err(|_| err_stt!(BadRequest))?;
        }

        builder.build()
    }

    pub fn proto(&self) -> Protocol {
        self.proto
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn param(&self, key: &str) -> Option<&str> {
        let Some(ref q) = self.query else { return None };

        q.param(key)
    }

    pub fn has_query(&self) -> bool {
        self.query.is_some()
    }

    pub fn route(&self) -> &Route {
        &self.route
    }

    // F: scrutinizer is a function that takes req and whatever else is necessary
    // generates the scrutinizing types
    // and then runs their Type::scrutunize()?
    // if no error is returned by the end then request is good
    // else if error we move to Message::Error variant from Message::Request
    // fn scrutinize<F, S: Scrutinizer>(&self, scrutinizer: F) -> Result<(), ErrorStatus>
    // where
    //     F: Fn(&Request, SocketMeta<'_>) -> Result<(), ErrorStatus>,
    // {
    //     todo!()
    // }
}
