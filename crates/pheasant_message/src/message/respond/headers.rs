use crate::Response;
use pheasant_headers::ToHeader;
use pheasant_headers::{ContentEncodingBits, EncodeBody, Encoding};
use pheasant_headers::{ContentType, SetContentLength, SetDate};

impl Response {
    fn encode(&mut self, encoder: Encoding) -> EncodeResponse {
        if let Some(ref mut body) = self.body {
            *body = encoder.encode(&body);
        }

        EncodeResponse::new(self, encoder.to_u8())
    }

    fn content_encoding(&mut self, encodings: &str) {
        self.headers
            .insert("Content-Encoding".to_owned(), encodings.to_owned());
    }
}

pub struct EncodeResponse<'a>(&'a mut Response, u8);

impl<'a> EncodeResponse<'a> {
    fn new(resp: &'a mut Response, enc: u8) -> Self {
        Self(resp, enc)
    }
}

impl<'a> EncodeResponse<'a> {
    fn encode(self, encoder: Encoding) -> Self {
        if let Some(ref mut body) = self.0.body {
            *body = encoder.encode(&body);
        }

        EncodeResponse::new(self.0, self.1 | encoder.to_u8())
    }

    pub fn content_encoding(self) -> &'a mut Response {
        let bits = ContentEncodingBits(self.1);

        self.0.content_encoding(bits.encoding_list());

        self.0
    }
}

impl Response {
    pub fn content_length(&mut self) -> &mut Self {
        let Some(ref body) = self.body else {
            return self;
        };

        let len = SetContentLength::new(&body);
        self.headers
            .insert("Content-Length".to_owned(), len.to_header());

        self
    }

    pub fn content_type(&mut self, mime: impl AsRef<str>) -> &mut Self {
        if self.body.is_none() {
            return self;
        }
        let ty = ContentType::new(mime.as_ref());

        self.headers.insert("Content-Type".into(), ty.to_header())
    }

    pub fn date(&mut self) -> &mut Self {
        self.headers.insert("Date".into(), SetDate.to_header())
    }
}

// NOTE ResponseCors are created and set by me
// the only cors the user touches/sets are the ResourceCors, which I borrow when generating the
// ResponseCors in Respond::intialize
//
// TODO need a trait to implement auto headers setting for specific response statues
// or better yet
// TODO impl Respond { fn insert_status_headers() match self.status {set status specific headers} }
