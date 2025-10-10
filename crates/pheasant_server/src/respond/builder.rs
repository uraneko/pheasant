use super::Respond;
use chrono::{DateTime, Utc};
use hashbrown::{HashMap, HashSet};
use mime::Mime;
use pheasant_core::{ErrorStatus, Protocol, Status, Successful};
use pheasant_headers::{ContentEncoding, Cookie, Encoding, Headers, RespondCors};
use pheasant_uri::Origin;

#[derive(Debug, Default)]
pub struct Builder<'a> {
    status: Status,
    proto: Protocol,
    body: Option<Vec<u8>>,
    headers: Headers,
    cookies: Option<HashSet<Cookie>>,
    cors: Option<RespondCors<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new(status: Status, proto: Protocol) -> Self {
        Self {
            status,
            proto,
            body: None,
            headers: Headers::default(),
            cors: None,
            cookies: None,
        }
    }

    // WARN use Scrutinizer for checks

    // checks that status and mime are set before building respond
    pub fn check_basic_fields(&self) -> Result<(), ErrorStatus> {
        todo!()
    }

    // checks that cors field is set when we handle a cross origin request
    pub fn check_cors_fields(&self) -> Result<(), ErrorStatus> {
        todo!()
    }

    pub fn serialize_headers(&self) -> Headers {
        todo!()
    }

    pub fn serialize_body(&self) -> Option<Vec<u8>> {
        todo!()
    }

    pub fn build(self) -> Result<Respond, ErrorStatus> {
        self.check_basic_fields()?;
        self.check_cors_fields()?;
        let headers = self.serialize_headers();
        let body = self.serialize_body();

        Ok(Respond {
            status: self.status,
            proto: self.proto,
            body,
            headers,
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
        let Some(body) = self.respond.body else {
            return self.respond;
        };

        let header = self.encoder.to_header();
        self.respond.body = Some(self.encoder.encode(body));
        self.respond.headers.header("Content-Encoding", header);

        self.respond
    }
}

impl<'a> Builder<'a> {
    pub fn content_length(mut self, len: usize) -> Self {
        self.headers.header("Content-Length", len);

        self
    }

    pub fn content_type(mut self, mime: Mime) -> Self {
        self.headers.header("Content-Type", mime);

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

    pub fn cors(mut self, cors: RespondCors<'a>) -> Self {
        self.cors = Some(cors);

        self
    }

    pub fn host(mut self, host: Origin) -> Self {
        self.headers.header("Host", host);

        self
    }

    pub fn date(mut self, date: DateTime<Utc>) -> Self {
        self.headers.header("Date", date);

        self
    }

    pub fn body(mut self, b: impl Into<Vec<u8>>) -> Self {
        self.body = Some(b.into());

        self
    }
}
