use super::Respond;
use crate::{HttpSocket, Request};
use chrono::Utc;
use mime::Mime;
use pheasant_core::{ErrorStatus, Protocol, Status, status};
use pheasant_middleware::{
    ContentEncoding, Cookie, Cookies, CorsConfigs, Encoding, Headers, RequestCors, RespondCors,
};
use pheasant_uri::{Origin, Route};

// TODO
// request building callback <- use your middlewares // parse request using http parser
// request routing callback <- use your middlewares // route or forward here
// respond preparing callback <- use your middlewares // if you need databases, borrow them here, set status here
// respond building callback <- use your middlewares // set respond headers and body here / maybe change status
//
// TODO
// try implementing the iterator pattern for middlewares

// TODO Builder<'a, T> <- T is inner type, user defined
// then user service becomes async fn service(builder:: respond::Builder<'_, T>) -> Respond {...}
// with both the Builder instance and a T instance being available to the user
// Generator::generate would also need to take a type generic T
// generator.generate::<T>(req, res, forward)
#[derive(Debug)]
pub struct Builder<'a> {
    pub proto: Protocol,
    /// the Respond body content
    pub status: Status,
    /// the Respond headers
    pub headers: Headers,
    /// the Respond protocol http1.1, 2, ws, ...
    pub body: &'a mut Vec<u8>,
}

impl<'a> Builder<'a> {
    pub fn new(
        status: Status,
        proto: Protocol,
        body: &'a mut Vec<u8>,
    ) -> Self {
        body.clear();
        Self {
            proto,
            status,
            body,
            headers: Headers::default(),
        }
    }

    pub fn forward(
        status: Status,
        proto: Protocol,
        body: &'a mut Vec<u8>,
        location: &'a Route,
        request: Request,
        socket: &'a HttpSocket,
    ) -> Self {
        Self {
            request,
            socket,
            status,
            proto,
            body,
            headers: Headers::from([("Location", location)]),
        }
    }

    pub fn error(
        status: ErrorStatus,
        proto: Protocol,
        body: &'a mut Vec<u8>,
        request: Request,
        socket: &'a HttpSocket,
    ) -> Self {
        Self {
            request,
            socket,
            status: status.into(),
            proto,
            body,
            headers: Headers::default(),
        }
    }

    pub fn build(self) -> Result<Respond<'a>, ErrorStatus> {
        Ok(Respond {
            status: self.status,
            proto: self.proto,
            body: self.body,
            headers: self.headers,
        })
    }
}

pub struct EncodeBody<'a> {
    encoder: ContentEncoding,
    respond: Builder<'a>,
}

impl<'a> EncodeBody<'a> {
    pub fn encoding(mut self, enc: Encoding) -> Self {
        self.encoder.encoding(enc);

        self
    }

    pub fn encode(mut self) -> Builder<'a> {
        if self.respond.body.is_empty() {
            return self.respond;
        };

        let header = self.encoder.to_header();
        *self.respond.body = self.encoder.encode(self.respond.body.to_vec());
        self.respond.headers.header("Content-Encoding", header);

        self.respond
    }
}

impl<'a> Builder<'a> {
    pub fn content_length(mut self) -> Self {
        self.headers.header("Content-Length", self.body.len());

        self
    }

    pub fn content_type(mut self, mime: &str) -> Self {
        self.headers
            .header("Content-Type", mime.parse::<Mime>().unwrap());

        self
    }

    /// e.g., respond.content_encoding("deflate").encoding("gzip").encode()
    pub fn content_encoding(self, encoding: Encoding) -> EncodeBody<'a> {
        EncodeBody {
            encoder: ContentEncoding::new(encoding),
            respond: self,
        }
    }

    pub fn cookie(mut self, cookie: Cookie) -> Self {
        self.headers.insert("Cookie", cookie);

        self
    }

    pub fn cookies(mut self, cookies: impl IntoIterator<Item = Cookie>) -> Self {
        self.headers
            .headers_from_iter("Set-Cookie", cookies.into_iter().map(|c| c.to_string()));

        self
    }

    pub fn host(mut self, host: Origin) -> Self {
        self.headers.header("Host", host);

        self
    }

    pub fn date(mut self) -> Self {
        self.headers.header("Date", Utc::now());

        self
    }

    pub fn server(mut self, server: &str) -> Self {
        self.headers.header("Server", server);

        self
    }

    pub fn body(self, b: impl Into<Vec<u8>>) -> Self {
        *self.body = b.into();

        self
    }
}
