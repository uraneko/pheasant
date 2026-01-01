use super::{
    AbsoluteUrl, Host, Path, PathRelativeUrl, Query, Scheme, SchemeRelativeUrl, Url, User,
};
use crate::Parse;
use crate::repeat_tfs;
use core::fmt::{Display, Formatter, Result as FmtRes};
use core::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Origin {
    scheme: Scheme,
    // user: Option<User>,
    host: Host,
    port: u16,
}

impl Display for Origin {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
        write!(f, "{:?}://{:?}:{}", self.scheme, self.host, self.port)
    }
}

impl From<AbsoluteUrl> for Origin {
    fn from(url: AbsoluteUrl) -> Self {
        let AbsoluteUrl {
            scheme, host, port, ..
        } = url;
        Self { scheme, host, port }
    }
}

impl From<SchemeRelativeUrl> for Origin {
    fn from(url: SchemeRelativeUrl) -> Self {
        let SchemeRelativeUrl { host, port, .. } = url;
        Self {
            scheme: Scheme::Https,
            host,
            port,
        }
    }
}

impl From<Url> for Option<Origin> {
    fn from(url: Url) -> Self {
        use Url::*;

        match url {
            Absolute(a) => Some(a.into()),
            SchemeRelative(sr) => Some(sr.into()),
            _ => None,
        }
    }
}

impl FromStr for Origin {
    type Err = <AbsoluteUrl as Parse>::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<AbsoluteUrl>().map(|url| url.into())
    }
}

impl Origin {
    // pub fn to_string(&self) -> String {
    //     if self.port != self.scheme.default_port() {
    //         format!("{}://{:?}:{}", self.scheme.as_str(), self.host, self.port)
    //     } else {
    //         format!("{}://{:?}", self.scheme.as_str(), self.host,)
    //     }
    // }

    pub fn new(scheme: Scheme, host: Host, port: Option<u16>) -> Self {
        Self {
            scheme,
            host,
            port: port.unwrap_or_else(|| scheme.default_port()),
        }
    }

    pub fn with_port(scheme: Scheme, host: Host, port: u16) -> Self {
        Self { scheme, host, port }
    }
}

impl From<PathRelativeUrl> for Path {
    fn from(url: PathRelativeUrl) -> Self {
        let PathRelativeUrl { path, .. } = url;

        path
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
        write!(f, "{}", {
            let mut p = self
                .segments()
                .iter()
                .fold("".to_owned(), |p, s| p + s + "/");
            p.pop();
            p
        })
    }
}

// impl From<AbsoluteUrl> for Path {}
// impl From<SchemeRelativeUrl> for Path {}

impl FromStr for Path {
    type Err = <PathRelativeUrl as Parse>::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<PathRelativeUrl>().map(|url| url.into())
    }
}

impl TryFrom<&[u8]> for Path {
    type Error = <PathRelativeUrl as Parse>::ParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        core::str::from_utf8(bytes)?.parse::<Self>()
    }
}

impl From<core::str::Utf8Error> for super::Error {
    fn from(err: core::str::Utf8Error) -> Self {
        Self::Str(err)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Resource {
    path: Path,
    query: Option<Query>,
}

impl Resource {
    pub fn path(&self) -> String {
        self.path.serialized()
    }

    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }

    pub fn disassemble(self) -> (Path, Option<Query>) {
        (self.path, self.query)
    }
}

impl From<Resource> for (Path, Option<Query>) {
    fn from(res: Resource) -> (Path, Option<Query>) {
        (res.path, res.query)
    }
}

impl From<PathRelativeUrl> for Resource {
    fn from(url: PathRelativeUrl) -> Self {
        let PathRelativeUrl { path, query, .. } = url;

        Self { path, query }
    }
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
        if let Some(q) = &self.query {
            write!(f, "{:?}?{:?}", self.path, q)
        } else {
            write!(f, "{:?}", self.path)
        }
    }
}

impl TryFrom<&[u8]> for Resource {
    type Error = <PathRelativeUrl as Parse>::ParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        core::str::from_utf8(bytes)?
            .parse::<PathRelativeUrl>()
            .map(|url| Self {
                path: url.path,
                query: url.query,
            })
    }
}

impl FromStr for Resource {
    type Err = <PathRelativeUrl as Parse>::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<PathRelativeUrl>().map(|url| url.into())
    }
}

repeat_tfs!(Resource);
