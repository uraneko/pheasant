use super::Request;
use hashbrown::{HashMap, HashSet};
use pheasant_core::{ErrorStatus, Method, Protocol};
use pheasant_headers::{Cookie, Cookies, Header, Headers, RequestCors};
use pheasant_uri::{Query, Resource, Route};

#[derive(Debug, PartialEq, Eq)]
pub struct Builder {
    pub(super) headers: Headers,
    pub(super) proto: Protocol,
    pub(super) method: Method,
    pub(super) resource: Resource,
    pub(super) body: Option<Vec<u8>>,
    // pub(super) cors: Option<RequestCors>,
    // pub(super) cookies: Option<HashSet<Cookie>>,
    // pub(super) query: Option<Query>,
}

impl Builder {
    pub fn body(&mut self, body: impl Into<Option<Vec<u8>>>) {
        self.body = body.into();
    }

    pub fn header(
        &mut self,
        header: Vec<u8>,
        field: Vec<u8>,
    ) -> Result<(), std::string::FromUtf8Error> {
        let header = String::from_utf8(header)?;
        let field = String::from_utf8(field)?;
        if header == "Cookie" {
            self.headers.headers(header, field);
        } else {
            self.headers.insert(header, field);
        }

        Ok(())
    }

    // pub fn headers(&mut self, headers: Vec<Token>) -> Result<(), std::string::FromUtf98Error> {
    //     headers.chunks(2).
    // }

    pub fn build(mut self) -> Result<Request, ErrorStatus> {
        let (route, query) = self.resource.into();
        let cors = self
            .headers
            .contains("Origin")
            .then(|| {
                let cors_headers = self.headers.slice_off([
                    "Origin",
                    "Access-Control-Request-Method",
                    "Access-Control-Request-Headers",
                ]);

                RequestCors::from_headers(cors_headers)
            })
            .transpose()?;

        let cookies = self
            .headers
            .extract("Cookie")
            .map(|cookies| Cookies::from_header(cookies))
            .transpose()?;
        let headers = self.headers;

        Ok(Request {
            headers,
            cookies,
            cors,
            proto: self.proto,
            method: self.method,
            route,
            query,
            body: self.body,
        })
    }
}
