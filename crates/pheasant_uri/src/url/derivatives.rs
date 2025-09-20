use crate::{Host, Scheme};
use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Origin {
    scheme: Scheme,
    host: Host,
    port: u16,
}

impl Origin {
    fn new(scheme: Scheme, host: Host, port: Option<u16>) -> Self {
        Self {
            scheme,
            host,
            port: port.unwrap_or_else(|| scheme.default_port()),
        }
    }
}

impl TryFrom<Url> for Origin {
    type Error = TransmuteError;

    fn try_from(mut url: Url) -> Result<Self, Self::Error> {
        let Some(scheme) = url.scheme() else {
            return Err(TransmuteError::OriginSchemeNotFound);
        };

        let Some(domain) = url.take_domain() else {
            return Err(TransmuteError::OriginDomainNotFound);
        };

        Ok(Self {
            scheme,
            host: domain.join("."),
            port: url.port(),
        })
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.sequence())
    }
}

impl FromStr for Origin {
    type Err = TransmuteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Url>().unwrap().interpret::<Self>()
    }
}

impl Origin {
    pub fn from_parts(scheme: Scheme, ip: IpAddr, port: u16) -> Self {
        Self {
            scheme,
            port: Some(port),
            domain: ip.to_string(),
        }
    }

    /// returns string repr of this domain
    pub fn domains(&self) -> std::str::Split<'_, char> {
        self.domain.split('/')
    }

    /// top level domain, a common example is com
    pub fn tld(&self) -> &str {
        self.domains().last().unwrap()
    }

    /// bottom level domain, a common example is www
    pub fn bld(&self) -> &str {
        self.domains().next().unwrap()
    }

    /// second level domain of this origin
    pub fn sld(&self) -> &str {
        self.domains().rev().skip(1).next().unwrap()
    }

    pub fn scheme(&self) -> Scheme {
        self.scheme
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    pub fn count(&self) -> usize {
        self.domains().count()
    }

    pub fn as_str(&self) -> &str {
        &self.domain
    }

    pub fn sequence(&self) -> String {
        let Some(port) = self.port else {
            return format!("{}://{}", self.scheme.as_str(), self.domain);
        };

        format!("{}://{}:{}", self.scheme.as_str(), self.domain, port)
    }

    pub fn is_any_origin(&self) -> bool {
        self.domain == "*"
    }
}

// resource.rs

use crate::{Path, Query};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Resource {
    path: Path,
    query: Option<Query>,
}

impl Resource {
    pub fn from_parts(path: Path, query: Option<Query>) -> Self {
        Self { path, query }
    }

    pub fn new(path: Path, query: Option<Query>) -> Self {
        Self { path, query }
    }
}

impl std::str::FromStr for Resource {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Url>().unwrap().interpret::<Self>()
    }
}

impl Resource {
    pub fn query(&self) -> Option<&Query> {
        let Some(ref query) = self.query else {
            return None;
        };

        Some(query)
    }

    pub fn contains_query(&self) -> bool {
        self.query.is_some()
    }

    pub fn params(&self) -> Option<&HashMap<String, String>> {
        let Some(ref query) = self.query else {
            return None;
        };

        Some(query.params())
    }

    pub fn attrs(&self) -> Option<&HashSet<String>> {
        let Some(ref query) = self.query else {
            return None;
        };

        Some(query.attrs())
    }

    pub fn contains_param(&self, k: &str) -> bool {
        let Some(params) = self.params() else {
            return false;
        };

        params.contains_key(k)
    }

    pub fn contains_attr(&self, k: &str) -> bool {
        let Some(attrs) = self.attrs() else {
            return false;
        };

        attrs.contains(k)
    }

    /// takes route from self
    pub fn take_route(&mut self) -> Route {
        std::mem::take(&mut self.route)
    }

    /// takes query from self
    pub fn take_query(&mut self) -> Option<Query> {
        std::mem::take(&mut self.query)
    }

    pub fn sequence(&self) -> String {
        let Some(ref query) = self.query else {
            return self.route.as_str().to_owned();
        };

        let mut seq = query.sequence();
        seq.insert_str(0, self.route.as_str());

        seq
    }
}

impl TryFrom<Url> for Resource {
    type Error = TransmuteError;

    fn try_from(mut url: Url) -> Result<Self, Self::Error> {
        let Some(path) = url.take_path() else {
            return Err(TransmuteError::RoutePathNotFound);
        };

        Ok(Self {
            route: Route::new(path),
            query: url.take_query(),
        })
    }
}

impl From<&Route> for TokenTree {
    fn from(route: &Route) -> Self {
        let mut ts = TS2::new();
        let ident = Ident::new("Route", Span::call_site());
        ts.append(ident);

        let lit = Group::new(
            Delimiter::Parenthesis,
            TokenTree::Literal(Literal::string(route.as_str())).into(),
        );
        ts.append(lit);

        let group = Group::new(Delimiter::None, ts);
        TokenTree::from(group)
    }
}
