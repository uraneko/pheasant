use core::str::Utf8Error;
use pheasant_http::{
    MaybeGlob, Method,
    request::{Header, contains_header, header_value},
};

// pub fn cors(resp: &mut Vec<u8>, status: &str) {
//     let headers = "access-control-allow-headers: *\n";
//     let origin = "access-control-allow-origin: 127.10.10.1:1024\n";
//     let methods = "access-control-allow-methods: HEAD, GET, OPTIONS\n";
//     *resp = format!("{}{}{}{}", status, headers, origin, methods);
// }

// TODO check value validity before pushing it to Cors

pub struct Cors {
    /// allowed methods
    /// can take a glob `*`
    methods: MaybeGlob<Vec<Method>>,
    /// allowed headers + headers expose
    /// can take a glob `*`
    headers: MaybeGlob<Vec<&'static str>>,
    /// this extends the list of client script (js) exposable response headers in a cors situation
    /// see `https://developer.mozilla.org/en-US/docs/Glossary/CORS-safelisted_response_header`
    /// for the list headers that are safelisted by default
    expose: MaybeGlob<Vec<&'static str>>,
    /// allowed origins
    /// can take a glob `*`
    origins: MaybeGlob<Vec<&'static str>>,
    /// are credentials allowed across origins
    credentials: bool,
    /// preflight request caching timeout
    max_age: Option<i64>,
}

// builder methods
impl Cors {
    pub fn new() -> Self {
        use Method::*;

        Self {
            credentials: false,
            max_age: None,
            methods: MaybeGlob::Value(vec![Head, Get, Options]),
            headers: MaybeGlob::Value(vec![]),
            expose: MaybeGlob::Value(vec![]),
            origins: MaybeGlob::Glob,
        }
    }

    pub fn methods(mut self, m: &[Method]) -> Self {
        match self.methods {
            MaybeGlob::Glob => self.methods = MaybeGlob::Value(m.to_vec()),
            MaybeGlob::Value(ref mut methods) => methods.extend(m),
        }

        self
    }

    pub fn method(mut self, method: Method) -> Self {
        match self.methods {
            MaybeGlob::Glob => self.methods = MaybeGlob::Value(vec![method]),
            MaybeGlob::Value(ref mut methods) => methods.push(method),
        }

        self
    }

    pub fn methods_glob(mut self) -> Self {
        self.methods = MaybeGlob::Glob;

        self
    }

    pub fn headers(mut self, h: &[&'static str]) -> Self {
        if h.iter().any(|h| *h == "*") && !self.headers.is_glob() {
            self.headers = MaybeGlob::Glob;

            return self;
        }

        let MaybeGlob::Value(headers) = self.headers.as_mut() else {
            self.headers = MaybeGlob::Value(h.to_vec());

            return self;
        };
        headers.extend(h);

        self
    }

    pub fn header(mut self, header: &'static str) -> Self {
        if header == "*" {
            if !self.headers.is_glob() {
                self.headers = MaybeGlob::Glob;
            }
        } else {
            match self.headers {
                MaybeGlob::Glob => self.headers = MaybeGlob::Value(vec![header]),
                MaybeGlob::Value(ref mut headers) => {
                    headers.clear();
                    headers.push(header);
                }
            }
        }

        self
    }

    /// the plural form of expose
    pub fn exposes(mut self, h: &[&'static str]) -> Self {
        if h.iter().any(|h| *h == "*") && !self.headers.is_glob() {
            self.headers = MaybeGlob::Glob;

            return self;
        }

        let MaybeGlob::Value(headers) = self.headers.as_mut() else {
            self.headers = MaybeGlob::Value(h.to_vec());

            return self;
        };
        headers.extend(h);

        self
    }

    /// adds a single header to the expose headers cors header
    pub fn expose(mut self, header: &'static str) -> Self {
        if header == "*" {
            if !self.headers.is_glob() {
                self.headers = MaybeGlob::Glob;
            }
        } else {
            match self.headers {
                MaybeGlob::Glob => self.headers = MaybeGlob::Value(vec![header]),
                MaybeGlob::Value(ref mut headers) => {
                    headers.clear();
                    headers.push(header);
                }
            }
        }

        self
    }

    pub fn origins(mut self, ori: &[&'static str]) -> Self {
        if ori.contains(&"*") && !self.origins.is_glob() {
            self.origins = MaybeGlob::Glob;

            return self;
        }

        let MaybeGlob::Value(origins) = self.origins.as_mut() else {
            self.origins = MaybeGlob::Value(ori.to_vec());

            return self;
        };
        origins.extend(ori);

        self
    }

    pub fn origin(mut self, origin: &'static str) -> Self {
        if origin == "*" {
            if !self.origins.is_glob() {
                self.origins = MaybeGlob::Glob;
            }
        } else {
            match self.origins {
                MaybeGlob::Glob => self.origins = MaybeGlob::Value(vec![origin]),
                MaybeGlob::Value(ref mut origins) => {
                    origins.clear();
                    origins.push(origin);
                }
            };
        }

        self
    }

    pub fn credentials(mut self, creds: bool) -> Self {
        self.credentials = creds;

        self
    }

    pub fn max_age(mut self, max_age: i64) -> Self {
        self.max_age = Some(max_age);

        self
    }
}

#[derive(Debug)]
pub enum Error {
    BadHeaderValue,
    ForbiddenOrigin,
    MissingRequestOrigin,
}

impl From<Utf8Error> for Error {
    fn from(_err: Utf8Error) -> Self {
        Self::BadHeaderValue
    }
}

pub fn allows_header(headers: &[&str], header: &str) -> bool {
    headers.iter().any(|h| *h == header)
}

pub fn allows_origin(origins: &[&str], origin: &str) -> bool {
    origins.contains(&origin)
}

// methods for checking request cors params against this struct's
impl Cors {
    /// checks if the passed header is allowed by these cors params
    pub fn allows_header(&self, header: &str) -> bool {
        let MaybeGlob::Value(ref headers) = self.headers else {
            return true;
        };

        headers.iter().any(|h| *h == header)
    }

    pub fn allows_origin(&self, origin: &str) -> bool {
        let MaybeGlob::Value(ref origins) = self.origins else {
            return true;
        };

        origins.contains(&origin)
    }
}

// methods for writing cors headers to the buffer
impl Cors {
    pub fn allow_origin(&self, value: &[u8], buffer: &mut Vec<u8>) -> Result<(), Error> {
        let MaybeGlob::Value(ref origins) = self.origins else {
            buffer.extend(b"access-control-allow-origin: *\n");

            return Ok(());
        };

        let origin = str::from_utf8(value)?;
        if !allows_origin(origins, origin) {
            return Err(Error::ForbiddenOrigin);
        }

        buffer.extend(b"access-control-allow-origin: ");
        buffer.extend(origin.as_bytes());
        buffer.push(10);

        Ok(())
    }

    pub fn allow_method(&self, value: &[u8], buffer: &mut Vec<u8>) -> Result<(), Error> {
        let MaybeGlob::Value(ref methods) = self.methods else {
            buffer.extend(b"access-control-allow-methods: *\n");

            return Ok(());
        };

        if methods.is_empty() {
            return Ok(());
        }

        // TODO str conversion is redundant
        // use method try from slice
        // NOTE this if block is needless
        // as the server we simply send the allowed methods and let the client
        // figure out its own permissions
        // if methods.contains(&str::from_utf8(value).map(|s| {
        //     s.trim()
        //         .to_lowercase()
        //         .parse::<Method>()
        //         .map_err(|_| Error::BadHeaderValue)
        // })??) {}

        buffer.extend(b"access-control-allow-methods: ");
        methods.iter().fold(&mut *buffer, |acc, m| {
            acc.extend(m.as_str().as_bytes());
            acc.push(b',');

            acc
        });

        buffer.pop();
        buffer.push(10);

        Ok(())
    }

    /// compares these params with the requested cors params
    /// writes the response cors headers
    pub fn allow_headers(&self, value: &[u8], buffer: &mut Vec<u8>) -> Result<(), Error> {
        let MaybeGlob::Value(ref headers) = self.headers else {
            buffer.extend(b"access-control-allow-headers: *\n");

            return Ok(());
        };

        if headers.is_empty() {
            return Ok(());
        }

        // NOTE it probably makes more sense to write all allowed headers
        // since client may cache that knowledge and use different headers later
        let mut state = 0;
        for header in str::from_utf8(value)?
            .split(|ch| ch == ',')
            .map(|h| h.trim())
            .filter(|h| allows_header(headers, h))
        {
            match state {
                // we do have matching headers
                // so we write the header field
                // to the buffer
                0 => state = 1,
                1 => {
                    buffer.extend(b"access-control-allow-headers: ");
                    buffer.extend(header.as_bytes());

                    state = 2;
                }
                2 => {
                    buffer.extend(header.as_bytes());
                    buffer.push(b',');
                }
                _ => (),
            }
        }
        buffer.pop();
        buffer.push(10);

        Ok(())
    }

    pub fn expose_headers(&self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        let MaybeGlob::Value(ref expose) = self.expose else {
            buffer.extend(b"access-control-expose-headers: *\n");

            return Ok(());
        };

        if expose.is_empty() {
            return Ok(());
        }

        expose.iter().fold(
            {
                buffer.extend(b"access-control-expose-headers: ");
                buffer
            },
            |acc, e| {
                acc.extend(e.as_bytes());
                acc.push(b',');
                acc
            },
        );

        Ok(())
    }

    pub fn allow_credentials(&self, buffer: &mut Vec<u8>) {
        if self.credentials {
            buffer.extend(b"access-control-allow-credentials: true\n");
        }
    }

    pub fn allow_max_age(&self, buffer: &mut Vec<u8>) {
        let Some(max_age) = self.max_age else {
            return;
        };

        buffer.extend(b"access-control-max-age: ");
        buffer.extend(max_age.to_string().as_bytes());
        buffer.push(10);
    }

    pub fn cors(&self, headers: &[Header], buffer: &mut Vec<u8>) -> Result<(), Error> {
        let Some(value) = header_value(headers, b"origin") else {
            return Err(Error::MissingRequestOrigin);
        };
        self.allow_origin(value, buffer)?;

        if let Some(value) = header_value(headers, b"access-control-request-method") {
            self.allow_method(value, buffer)?;
        }

        if let Some(value) = header_value(headers, b"access-control-request-headers") {
            self.allow_headers(value, buffer)?;
        }

        self.allow_credentials(buffer);
        self.allow_max_age(buffer);

        Ok(())
    }
}
