extern crate alloc;
use alloc::{
    borrow::ToOwned,
    format,
    string::{String, ToString},
};
use chrono::{DateTime, TimeDelta, Utc};
use core::fmt::Debug;
use core::str::FromStr;
use hashbrown::HashSet;
use pheasant_core::{ClientError, ErrorStatus, ServerError};

use crate::{FromHeader, FromHeaders, HttpResult, ToHeader, ToHeaders};

impl ToHeaders for HashSet<Cookie> {
    fn to_headers(&self) -> impl Iterator<Item = (&str, String)> {
        self.iter().map(|cookie| ("Set-Cookie", cookie.to_header()))
    }
}

impl ToHeader for HashSet<Cookie> {
    fn to_header(&self) -> String {
        let mut header = self.iter().fold("".to_owned(), |acc, cookie| {
            acc + &cookie.to_string() + "; "
        });

        header.pop();
        header.pop();

        header
    }
}

// DOCS
// the Cookie header is client specific
// while the Set-Cookie header is server only

// WARN browsers' session restore feature also restores session cookies
// NOTE
// if no expires or max-age attrs are set then the cookie is auto expired at browser session shutdown
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Cookie {
    expires: Option<DateTime<Utc>>,
    max_age: Option<TimeDelta>,
    http_only: bool,
    // requires the Secure attr
    partitioned: bool,
    secure: bool,
    path: Option<String>,
    domain: Option<String>,
    same_site: Option<SameSite>,
    key: String,
    val: String,
}

const EXTS: [&str; 7] = [
    "Domain",
    "Expires",
    "HttpOnly",
    "Max-Age",
    "Partitioned",
    "Path",
    "SameSite",
];
fn split_on_key(s: &mut String, delim: &str) -> Option<String> {
    if !EXTS.iter().any(|ext| s.starts_with(ext)) {
        return None;
    }

    // WARN this should actually return an Err not None
    s.find(delim).map(|idx| s.drain(..idx).collect())
}

fn split_on_val(s: &mut String, delim: &str) -> HttpResult<String> {
    if !EXTS.iter().any(|ext| s.starts_with(ext)) {
        return Err(ErrorStatus::Client(ClientError::BadRequest));
    }

    s.find(delim)
        .map(|idx| s.drain(..idx).collect())
        .ok_or_else(|| ErrorStatus::Client(ClientError::BadRequest))
}

fn take_key_val(s: &mut String) -> Option<HttpResult<[String; 2]>> {
    if s.is_empty() {
        return None;
    }

    if let [Ok(key), Ok(val)] = [
        s.find('=')
            .map(|idx| s.drain(..idx).collect())
            .ok_or_else(|| ErrorStatus::Server(ServerError::NotImplemented)),
        s.find("; ")
            .map(|idx| s.drain(..idx).collect())
            .ok_or_else(|| ErrorStatus::Server(ServerError::NotImplemented)),
    ] {
        Some(Ok([key, val]))
    } else {
        Some(Err(ErrorStatus::Server(ServerError::NotImplemented)))
    }
}

// WARN HTTP/2 allows requests to have many Cookie headers for compression optimizations
// HTTP/1.1 tho, doesnt allow this feature
impl FromHeaders<'_> for HashSet<Cookie> {
    type Headers = HashSet<String>;

    fn from_headers(mut h: HashSet<String>) -> HttpResult<Self> {
        let mut err = false;
        if h.is_empty() {
            return Err(ErrorStatus::Server(ServerError::NotImplemented));
        }

        let mut iter = h.into_iter().map(|mut header| {
            if let Some(kv) = take_key_val(&mut header) {
                let [ref k, ref v] = kv?;

                Ok(Cookie::new(k, v).fill_out(&mut header))
            } else {
                err = true;
                Err(ErrorStatus::Server(ServerError::NotImplemented))
            }
        });

        // TODO
        // if err {
        //     let Some(Err(err)) = iter.find(|c| c.is_err()) else {
        //         unreachable!("the logic commands you to stop");
        //     };
        //
        //     return Err(err);
        // }

        iter.map(|c| c.unwrap()).collect()
    }
}

impl FromHeader for HashSet<Cookie> {
    fn from_header(mut header: String) -> HttpResult<Self> {
        let mut set = HashSet::new();
        while let Some(kv) = take_key_val(&mut header) {
            let [ref k, ref v] = kv?;
            set.insert(Cookie::new(k, v).fill_out(&mut header)?);
        }

        Ok(set)
    }
}

impl Cookie {
    pub fn new(k: &str, v: &str) -> Self {
        Self {
            key: k.to_owned(),
            val: v.to_owned(),
            ..Default::default()
        }
    }

    // adds an expiration datetime to the cookie
    // takes a datetim after which the cookie should be expired
    // this sets the Expires cookie attribute
    //
    // WARN the server sets this attribute using its own clock, which may differ from the client
    // side's clock,
    // it is advised to use the less error prone Max-Age attr instead
    pub fn expires(&mut self, datetime: DateTime<Utc>) -> &mut Self {
        self.expires = Some(datetime);

        self
    }

    // adds an expiration datetime to the cookie
    // takes a duration after which the cookie should be expired
    // this sets the Max-Age cookie attribute
    pub fn max_age(&mut self, delta: TimeDelta) -> &mut Self {
        self.max_age = Some(delta.into());

        self
    }

    // if switch is set to true then the client side can not access this cookie using javascript
    pub fn http_only(&mut self, switch: bool) -> &mut Self {
        self.http_only = switch;

        self
    }

    /// sets the request uri path that triggers this cookie to be send with the request
    /// sub paths will also be considered as matches
    pub fn path(&mut self, path: &str) -> &mut Self {
        self.path = Some(path.into());

        self
    }

    /// sets the domain to which the client will send this cookie with requests
    /// if the domain value does not include the cookie defining server, then
    /// the cookie is rejected
    /// that includes server subdomains;
    /// example.com can not send a cookie with Domain=foo.example.com
    pub fn domain(&mut self, domain: &str) -> &mut Self {
        self.domain = Some(domain.into());

        self
    }

    pub fn same_site<SS>(&mut self, ss: SS) -> &mut Self
    where
        SS: TryInto<SameSite>,
        <SS as TryInto<SameSite>>::Error: core::fmt::Debug,
    {
        self.same_site = Some(ss.try_into().unwrap());

        self
    }

    /// switch for whether the cookie should be stored in partitioned storage
    /// requires the Secure attr
    pub fn partitioned(&mut self, switch: bool) -> &mut Self {
        self.partitioned = switch;

        self
    }

    /// only send this cookie with requests coming from the `https:` scheme
    pub fn secure(&mut self, switch: bool) -> &mut Self {
        self.secure = switch;

        self
    }
}

impl Cookie {
    fn fill_out(mut self, header: &mut String) -> HttpResult<Self> {
        while let Some(ref ext) = split_on_key(header, "=") {
            match ext.as_str() {
                "Domain" => self.domain(&split_on_val(header, "; ")?),
                "Expires" => self.expires(
                    split_on_val(header, "; ")?
                        .parse::<DateTime<Utc>>()
                        .unwrap(),
                ),
                "HttpOnly" => self.http_only(true),
                "Max-Age" => self.max_age(TimeDelta::seconds(
                    split_on_val(header, "; ")?.parse::<i64>().unwrap(),
                )),
                "Partitioned" => self.partitioned(true),
                "Path" => self.path(split_on_val(header, "; ")?.as_str()),
                "SameSite" => self.same_site(split_on_val(header, "; ")?.parse::<SameSite>()?),
                "Secure" => self.secure(true),
                _ => return Err(ErrorStatus::Server(ServerError::NotImplemented)),
            };
        }

        Ok(self)
    }
}

impl ToHeader for Cookie {
    fn to_header(&self) -> String {
        self.to_string()
    }
}

impl ToString for Cookie {
    fn to_string(&self) -> String {
        let mut cookie = format!("{}={}", self.key, self.val);
        let mut temp;
        if let Some(ma) = self.max_age {
            let ma = ma.num_seconds();
            temp = format!("; Max-Age={} ", ma);
            cookie.push_str(&temp);
        }

        if let Some(exp) = self.expires {
            temp = format!("; Expires={} ", exp);
            cookie.push_str(&temp)
        }

        if self.http_only {
            cookie.push_str("; HttpOnly");
        }

        if self.secure {
            cookie.push_str("; Secure");

            if self.partitioned {
                cookie.push_str("; Partitioned");
            }
        }

        if let Some(path) = &self.path {
            temp = format!("; Path={}", path);
            cookie.push_str(&temp);
        }

        if let Some(domain) = &self.domain {
            temp = format!("; Domain={}", domain);
            cookie.push_str(&temp);
        }

        if let Some(ss) = &self.same_site {
            temp = format!("; SameSite={:?}", ss);
            cookie.push_str(&temp);
        }

        cookie
    }
}

// NOTE the same domain with a different scheme is considered a different domain
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum SameSite {
    // only send this cookie on requests made to the same site that defined it
    Strict = 1,
    // same as strict but also includes cross site requests that
    // - are top level navigation requests; i.e., they move pages
    // - use a safe http method (dont set data in the server)
    Lax = 2,
    // the cookie is send with both same site and cross site requests
    // requires the secure attr
    #[default]
    None = 0,
}

impl TryFrom<u8> for SameSite {
    type Error = ErrorStatus;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::None),
            1 => Ok(Self::Strict),
            2 => Ok(Self::Lax),
            _ => Err(ErrorStatus::Server(ServerError::NotImplemented)),
        }
    }
}

impl FromStr for SameSite {
    type Err = ErrorStatus;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Strict" => Ok(Self::Strict),
            "Lax" => Ok(Self::Lax),
            "None" => Ok(Self::None),
            _ => Err(ErrorStatus::Server(ServerError::NotImplemented)),
        }
    }
}
