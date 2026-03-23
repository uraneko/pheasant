use core::str::Utf8Error;
use embedded_io::{ErrorType, Write};
use hashbrown::HashSet;
use pheasant_prologue::{Header, MaybeGlob, Method, header_value};

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
    methods: MaybeGlob<HashSet<Method>>,
    /// allowed headers + headers expose
    /// can take a glob `*`
    headers: MaybeGlob<HashSet<&'static str>>,
    /// this extends the list of client script (js) exposable response headers in a cors situation
    /// see `https://developer.mozilla.org/en-US/docs/Glossary/CORS-safelisted_response_header`
    /// for the list headers that are safelisted by default
    expose: MaybeGlob<HashSet<&'static str>>,
    /// allowed origins
    /// can take a glob `*`
    origins: MaybeGlob<HashSet<&'static str>>,
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
            methods: MaybeGlob::Value(HashSet::from([Head, Get, Options])),
            headers: MaybeGlob::Value(HashSet::new()),
            expose: MaybeGlob::Value(HashSet::new()),
            origins: MaybeGlob::Glob,
        }
    }

    pub fn methods(mut self, m: &[Method]) -> Self {
        match self.methods {
            MaybeGlob::Glob => self.methods = MaybeGlob::Value(m.into_iter().map(|m| *m).collect()),
            MaybeGlob::Value(ref mut methods) => methods.extend(m),
        }

        self
    }

    pub fn method(mut self, method: Method) -> Self {
        match self.methods {
            MaybeGlob::Glob => self.methods = MaybeGlob::Value(HashSet::from([method])),
            MaybeGlob::Value(ref mut methods) => _ = methods.insert(method),
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
            self.headers = MaybeGlob::Value(h.into_iter().map(|h| *h).collect());

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
                MaybeGlob::Glob => self.headers = MaybeGlob::Value([header].into_iter().collect()),
                MaybeGlob::Value(ref mut headers) => {
                    headers.clear();
                    headers.insert(header);
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
            self.headers = MaybeGlob::Value(h.into_iter().map(|h| *h).collect());

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
                MaybeGlob::Glob => self.headers = MaybeGlob::Value(HashSet::from([header])),
                MaybeGlob::Value(ref mut headers) => {
                    headers.clear();
                    headers.insert(header);
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
            self.origins = MaybeGlob::Value(ori.into_iter().map(|o| *o).collect());

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
                MaybeGlob::Glob => self.origins = MaybeGlob::Value(HashSet::from([origin])),
                MaybeGlob::Value(ref mut origins) => {
                    origins.clear();
                    origins.insert(origin);
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
    IoFailure,
}

impl From<Utf8Error> for Error {
    fn from(_err: Utf8Error) -> Self {
        Self::BadHeaderValue
    }
}

pub fn allows_header(headers: &HashSet<&str>, header: &str) -> bool {
    headers.iter().any(|h| *h == header)
}

pub fn allows_origin(origins: &HashSet<&str>, origin: &str) -> bool {
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

    pub fn allows_method(&self, method: &str) -> bool {
        let MaybeGlob::Value(ref methods) = self.methods else {
            return true;
        };

        methods.iter().any(|m| m.as_str() == method)
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
    pub fn allow_origin<W>(&self, value: &[u8], buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        Error: From<<W as ErrorType>::Error>,
    {
        let MaybeGlob::Value(ref origins) = self.origins else {
            buffer.write(b"access-control-allow-origin: *\n")?;

            return Ok(());
        };

        let origin = str::from_utf8(value)?;
        if !allows_origin(origins, origin) {
            return Err(Error::ForbiddenOrigin);
        }

        buffer.write(b"access-control-allow-origin: ")?;
        buffer.write(origin.as_bytes())?;
        buffer.write(&[10])?;

        Ok(())
    }

    pub fn allow_methods<W>(&self, _value: &[u8], buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        Error: From<<W as ErrorType>::Error>,
    {
        let MaybeGlob::Value(ref methods) = self.methods else {
            buffer.write(b"access-control-allow-methods: *\n")?;

            return Ok(());
        };

        if methods.is_empty() {
            return Ok(());
        }
        buffer.write(b"access-control-allow-methods: ")?;

        // TODO str conversion is redundant
        // use method try from slice
        // NOTE this if block is needless
        // as the server we simply send the allowed methods and let the client
        // figure out its own permissions
        //
        // if methods.contains(&str::from_utf8(value).map(|s| {
        //     s.trim()
        //         .to_lowercase()
        //         .parse::<Method>()
        //         .map_err(|_| Error::BadHeaderValue)
        // })??) {}
        let mut size = methods.len();
        let mut iter = methods.into_iter();
        while let Some(m) = iter.next() {
            buffer.write(m.as_str().as_bytes())?;
            if size >= 1 {
                buffer.write(b", ")?;
            }
            size -= 1;
        }
        buffer.write(&[10])?;

        Ok(())
    }

    /// compares these params with the requested cors params
    /// writes the response cors headers
    pub fn allow_headers<W>(&self, value: &[u8], buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        Error: From<<W as ErrorType>::Error>,
    {
        let MaybeGlob::Value(ref headers) = self.headers else {
            buffer.write(b"access-control-allow-headers: *\n")?;

            return Ok(());
        };

        if headers.is_empty() {
            return Ok(());
        }
        // NOTE it probably makes more sense to write all allowed headers
        // since client may cache that knowledge and use different headers later

        // WARN str are unnecessary
        let mut headers = str::from_utf8(value)?
            .split(|ch| ch == ',')
            .map(|h| h.trim())
            .filter(|h| allows_header(headers, h))
            .peekable();

        if headers.peek().is_none() {
            return Ok(());
        }

        buffer.write(b"access-control-allow-headers: ")?;
        while let Some(header) = headers.next() {
            buffer.write(header.as_bytes())?;
            if headers.peek().is_some() {
                buffer.write(b", ")?;
            }
        }
        buffer.write(&[10])?;

        Ok(())
    }

    pub fn expose_headers<W>(&self, buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        Error: From<<W as ErrorType>::Error>,
    {
        let MaybeGlob::Value(ref expose) = self.expose else {
            buffer.write(b"access-control-expose-headers: *\n")?;

            return Ok(());
        };

        if expose.is_empty() {
            return Ok(());
        }
        buffer.write(b"access-control-expose-headers: ")?;

        let mut size = expose.len();
        let mut iter = expose.into_iter();
        while let Some(e) = iter.next() {
            buffer.write(e.as_bytes())?;
            if size >= 1 {
                buffer.write(b", ")?;
            }

            size -= 1;
        }

        Ok(())
    }

    pub fn allow_credentials<W>(&self, buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        Error: From<<W as ErrorType>::Error>,
    {
        if self.credentials {
            buffer.write(b"access-control-allow-credentials: true\n")?;
        }

        Ok(())
    }

    pub fn allow_max_age<W>(&self, buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        Error: From<<W as ErrorType>::Error>,
    {
        use num_into_ascii::NumToAscii;

        let Some(max_age) = self.max_age else {
            return Ok(());
        };

        buffer.write(b"access-control-max-age: ")?;
        let (slice, size) = max_age.ascii_bytes();
        buffer.write(&slice[..size])?;
        buffer.write(&[10])?;

        Ok(())
    }

    pub fn cors<W>(&self, headers: &[Header], buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        Error: From<<W as ErrorType>::Error>,
    {
        // if there is no origin then we assume the request is not a cors one
        // and we early return
        let Some(value) = header_value(headers, b"origin") else {
            // return Err(Error::MissingRequestOrigin);
            return Ok(());
        };
        self.allow_origin(value, buffer)?;

        if let Some(value) = header_value(headers, b"access-control-request-method") {
            self.allow_methods(value, buffer)?;
        }

        if let Some(value) = header_value(headers, b"access-control-request-headers") {
            self.allow_headers(value, buffer)?;
        }

        self.allow_credentials(buffer)?;
        self.allow_max_age(buffer)?;

        Ok(())
    }

    pub fn cors_with_cookies<W>(&self, headers: &[Header], buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
        Error: From<<W as ErrorType>::Error>,
    {
        // if there is no origin then we assume the request is not a cors one
        // and we early return
        let Some(value) = header_value(headers, b"origin") else {
            // return Err(Error::MissingRequestOrigin);
            return Ok(());
        };
        self.allow_origin(value, buffer)?;
        self.allow_methods(value, buffer)?;
        self.allow_headers(value, buffer)?;
        self.allow_credentials(buffer)?;
        self.allow_max_age(buffer)?;

        Ok(())
    }
}
