extern crate alloc;
use alloc::{
    borrow::ToOwned,
    format,
    string::{String, ToString},
};
use chrono::TimeDelta;
use core::fmt::{self, Debug, Display, Formatter};
use hashbrown::{HashMap, HashSet};

use crate::{FromHeader, FromHeaders, HttpResult, ToHeader, ToHeaders};
use pheasant_core::{ClientError, ErrorStatus, Method, WildCardish};
use pheasant_uri::Origin;

// TODO Timing-Allow-Origin header

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestCors {
    /// the cors req origin, client MUST provide this header
    origin: WildCardish<Origin>,
    /// client MAY send this
    headers: Option<HashSet<String>>,
    /// client MUST send this
    method: Method,
}

impl<'a> FromHeaders<'a> for RequestCors {
    type Headers = &'a mut HashMap<String, String>;

    fn from_headers(h: &mut HashMap<String, String>) -> HttpResult<Self> {
        let (Some(origin), Some(method)) = (
            h.remove("Origin")
                .map(|o| FromHeader::from_header(o).unwrap()),
            h.remove("Access_Control_Request_Method")
                .map(|m| FromHeader::from_header(m).unwrap()),
        ) else {
            // NOTE could also use 422 maybe
            return Err(ErrorStatus::Client(ClientError::BadRequest));
        };

        let headers = h
            .remove("Access_Control_Request_Headers")
            .map(|h| FromHeader::from_header(h).unwrap());

        Ok(Self {
            origin,
            method,
            headers,
        })
    }
}

impl FromHeader for WildCardish<Origin> {
    fn from_header(h: String) -> HttpResult<Self> {
        if h == "*" {
            return Ok(WildCardish::Glob);
        }

        Ok(WildCardish::Value(h.parse::<Origin>().unwrap()))
    }
}

impl FromHeader for Method {
    fn from_header(h: String) -> HttpResult<Self> {
        Ok(h.parse::<Method>().unwrap())
    }
}

impl FromHeader for HashSet<String> {
    fn from_header(h: String) -> HttpResult<Self> {
        Ok(h.split(',').map(|s| s.trim().to_owned()).collect())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ResourceCors {
    /// allowed cors req headers
    headers: HashSet<String>,
    /// the server allows these headers to be exposed to the used in the client side
    /// see https://developer.mozilla.org/en-US/docs/Glossary/CORS-safelisted_response_header
    expose: Option<HashSet<String>>,
    /// set of whitelisted origins or glob '*' to allow any origin
    origins: WildCardish<HashSet<Origin>>,
    /// allow credentials for this cors requests
    credentials: bool,
    /// max-age dictates how long the response of an options request can be cached for
    max_age: Option<TimeDelta>,
}

impl ResourceCors {
    /// no unwrap in this function is bad or dangerous, when used as/where intended
    ///
    /// this function was made to be used inside the http methods macros
    ///
    /// the args it takes are stringified from the correct values parsed and error handled in the
    /// macro
    pub fn macro_checked(
        method: Method,
        headers: HashSet<String>,
        expose: Option<HashSet<String>>,
        origins: WildCardish<HashSet<Origin>>,
        credentials: bool,
        max_age: Option<i64>,
    ) -> Self {
        Self {
            credentials,
            method,
            headers,
            expose,
            origins,
            max_age: max_age.map(|ma| TimeDelta::new(ma, 0).unwrap()),
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn origin(&mut self, origin: Origin) -> &mut Self {
        if let WildCardish::Value(ref mut origins) = self.origins {
            origins.insert(origin);
        }

        self
    }

    pub fn origins(&mut self, origins: HashSet<Origin>) -> &mut Self {
        self.origins = WildCardish::Value(origins);

        self
    }

    /// allows any origin to make the cors request
    /// using the glob operator
    pub fn glob_origin(&mut self) -> &mut Self {
        self.origins = WildCardish::Glob;

        self
    }

    pub fn method(&mut self, method: Method) -> &mut Self {
        self.method = method;

        self
    }

    pub fn header(&mut self, header: String) -> &mut Self {
        self.headers.insert(header);

        self
    }

    pub fn headers<I>(&mut self, headers: I) -> &mut Self
    where
        I: IntoIterator<Item = String>,
    {
        self.headers.extend(headers);

        self
    }

    pub fn expose(&mut self, header: String) -> &mut Self {
        self.expose.as_mut().map(|ex| ex.insert(header));

        self
    }

    pub fn exposes<I>(&mut self, headers: I) -> &mut Self
    where
        I: IntoIterator<Item = String>,
    {
        self.expose.as_mut().map(|ex| ex.extend(headers));

        self
    }

    pub fn max_age<T>(&mut self, ma: T) -> &mut Self
    where
        T: Into<TimeDelta>,
    {
        self.max_age.map(|_| ma.into());

        self
    }

    pub fn credentials(&mut self, creds: bool) -> &mut Self {
        self.credentials = creds;

        self
    }
}

impl ResourceCors {
    pub fn allows_access_for_origin(&self, origin: &str) -> bool {
        if origin == "*" {
            return true;
        }

        match self
            .origins
            .as_ref()
            .map(|inner| inner.iter().any(|o| o.as_str() == origin))
        {
            Some(b) => b,
            None => false,
        }
    }

    pub fn method_cpy(&self) -> Method {
        self.method
    }

    pub fn expose_ref(&self) -> Option<&HashSet<String>> {
        self.expose.as_ref()
    }

    pub fn max_age_cpy(&self) -> Option<i64> {
        self.max_age.as_ref().map(|ma| ma.num_seconds())
    }

    pub fn headers_iter(&self) -> impl Iterator<Item = &str> {
        self.headers.iter().map(|s| s.as_str())
    }

    pub fn origin_ref(&self, origin: &str) -> WildCardish<&Origin> {
        if origin == "*" {
            return WildCardish::Glob;
        }

        let Some(Some(origin)) = self
            .origins
            .as_ref()
            .map(|inner| inner.iter().find(|o| o.as_str() == origin))
        else {
            unreachable!("origin is not allowed by the registered cors")
        };

        WildCardish::Value(origin)
    }
}

pub struct ResponseCors<'a> {
    methods: &'a [Method],
    headers: &'a HashSet<String>,
    expose: Option<&'a HashSet<String>>,
    origin: WildCardish<&'a Origin>,
    credentials: bool,
    max_age: Option<String>,
}

impl<'a> ResponseCors<'a> {
    fn from_service(
        cors: &'a ResourceCors,
        origin: &str,
        resource_cors_methods: &'a [Method],
    ) -> HttpResult<Self> {
        if !cors.allows_access_for_origin(origin) {
            return Err(ErrorStatus::Client(ClientError::Forbidden));
        }

        Ok(Self {
            methods: resource_cors_methods,
            max_age: cors.max_age.map(|ma| ma.to_string()),
            credentials: cors.credentials,
            origin: cors.origin_ref(origin),
            headers: &cors.headers,
            expose: cors.expose.as_ref(),
        })
    }
}

impl<'a> ToHeaders for ResponseCors<'a> {
    fn to_headers(&self) -> impl Iterator<Item = (&str, String)> {
        Some((
            "Access-Control-Allow-Origin",
            self.origin.to_header().into(),
        ))
        .into_iter()
        .chain(Some((
            "Access-Control-Allow-Methods",
            self.methods.to_header().into(),
        )))
        .chain(Some((
            "Access-Control-Allow-Headers",
            self.headers.to_header().into(),
        )))
        .chain(self.expose.map(|expose| {
            (
                "Access-Control-Expose-Headers",
                expose.to_header().to_header().into(),
            )
        }))
        .chain(Some((
            "Access-Control-Allow-Credintials",
            self.credentials.to_header().into(),
        )))
        .chain(
            self.max_age
                .as_ref()
                .map(|ma| ("Access-Control-Max-Age", ma.to_header().into())),
        )
    }
}

impl ToHeader for bool {
    fn to_header(&self) -> &str {
        if *self { "true" } else { "false" }
    }
}

impl ToHeader for Method {
    fn to_header(&self) -> &str {
        self.as_str()
    }
}

impl<'a> ToHeader for &'a [Method] {
    fn to_header(&self) -> String {
        let mut s = self
            .into_iter()
            .fold("".to_owned(), |acc, m| acc + m.as_str() + ", ");
        s.pop();
        s.pop();

        s
    }
}

impl ToHeader for HashSet<String> {
    fn to_header(&self) -> String {
        let mut s = self
            .into_iter()
            .fold("".to_owned(), |acc, h| acc + h.as_str() + ", ");
        s.pop();
        s.pop();

        s
    }
}

impl ToHeader for WildCardish<&Origin> {
    fn to_header(&self) -> &str {
        match self {
            Self::Glob => "*",
            Self::Value(o) => o.as_str(),
        }
    }
}

impl ToHeader for Origin {
    fn to_header(&self) -> &str {
        self.as_str()
    }
}

impl ToHeader for String {
    fn to_header(&self) -> &str {
        &self
    }
}

// PERF the expose field should be: expose: Vec<&str>
// // <- self referencing from the headers field
// should use pin

impl Display for ResourceCors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
