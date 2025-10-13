use super::{Respond, ScrutinizeCors};
use chrono::{TimeDelta, Utc};
use hashbrown::HashSet;
use mime::Mime;
use pheasant_core::{ErrorStatus, Protocol, Status, Successful};
use pheasant_headers::{ContentEncoding, Cookie, Encoding, Headers, RespondCors};
use pheasant_uri::Origin;

#[derive(Debug, Default)]
pub struct Builder<'a> {
    status: Status,
    proto: Protocol,
    body: Option<Vec<u8>>,
    cross_origin: bool,
    headers: Headers,
    cookies: Option<HashSet<Cookie>>,
    cors: Option<RespondCors<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new(status: Status, proto: Protocol, cross_origin: bool) -> Self {
        Self {
            status,
            proto,
            cross_origin,
            body: None,
            headers: Headers::default(),
            cors: None,
            cookies: None,
        }
    }

    // WARN use Scrutinizer for checks

    pub fn serialize_headers(&self) -> Headers {
        todo!()
    }

    pub fn serialize_body(&self) -> Option<Vec<u8>> {
        todo!()
    }

    pub fn build(self) -> Result<Respond, ErrorStatus> {
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
    pub fn content_length(mut self) -> Self {
        self.headers.header(
            "Content-Length",
            self.body.as_ref().map(|b| b.len()).unwrap_or_default(),
        );

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

    pub fn cors(mut self, cors: RespondCors<'a>) -> Self {
        self.cors = Some(cors);

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

    pub fn body(mut self, b: impl Into<Vec<u8>>) -> Self {
        self.body = Some(b.into());

        self
    }
}
