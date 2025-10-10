extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::string::String;

use super::CorsHeader;
use chrono::TimeDelta;
use hashbrown::HashSet;
use pheasant_core::WildCardish;
use pheasant_uri::Origin;

impl CorsHeader {
    fn new(header: String, expose: bool) -> Self {
        Self { header, expose }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CorsConfigs {
    /// allowed cors req headers
    pub headers: HashSet<CorsHeader>,
    /// the server allows these headers to be exposed to the used in the client side
    /// see https://developer.mozilla.org/en-US/docs/Glossary/CORS-safelisted_response_header
    // expose: Option<HashSet<String>>,
    /// set of whitelisted origins or glob '*' to allow any origin
    pub origins: WildCardish<HashSet<Origin>>,
    /// allow credentials for this cors requests
    pub credentials: bool,
    /// max-age dictates how long the response of an options request can be cached for
    pub max_age: Option<TimeDelta>,
}

impl CorsConfigs {
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

    pub fn header(&mut self, header: String, expose: bool) -> &mut Self {
        self.headers.insert(CorsHeader { header, expose });

        self
    }

    pub fn headers<I>(&mut self, headers: I) -> &mut Self
    where
        I: IntoIterator<Item = CorsHeader>,
    {
        self.headers.extend(headers);

        self
    }

    // pub fn expose(&mut self, header: &str, expose: bool) -> &mut Self {
    //     self.headers
    //         .iter()
    //         .filter(|h| h.header == header)
    //         .for_each(|h| h.expose = expose);
    //
    //     self
    // }

    // pub fn expose_batch(&mut self, headers: &[&str], expose: bool) -> &mut Self {
    //     headers
    //         .into_iter()
    //         .filter(|h| self.headers.iter().any(|hdr| hdr.header == **h))
    //         .for_each(|s| {
    //             let Some(mut header) = self.headers.iter().find(|h| h.header == *s) else {
    //                 panic!()
    //             };
    //             self.headers.remove(header);
    //             header.expose = expose;
    //
    //             self.headers.insert(*header);
    //         });
    //
    //     self
    // }

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

    // TODO better not compare str but actual fields in origin
    // origin == other_origin
    pub fn allows_origin(&self, origin: WildCardish<&Origin>) -> bool {
        if origin.is_glob() {
            return true;
        }

        match self
            .origins
            .maybe_ref()
            .map(|inner| inner.iter().any(|o| Some(&o) == origin.maybe_ref()))
        {
            Some(bool) => bool,
            None => false,
        }
    }

    // pub fn origin_ref(&self, origin: &WildCardish<Origin>) -> WildCardish<&Origin> {
    //     if origin.is_glob() {
    //         return WildCardish::Glob;
    //     }
    //
    //     let Some(Some(origin)) = self.origins.maybe_ref().map(|inner| {
    //         inner
    //             .iter()
    //             .find(|o| WildCardish::Value(*o) == origin.as_ref())
    //     }) else {
    //         unreachable!("origin is not allowed by the registered cors")
    //     };
    //
    //     WildCardish::Value(origin)
    // }
}

impl core::fmt::Display for CorsConfigs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
