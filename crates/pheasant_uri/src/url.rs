//! parse step
//! parses the components into a uri
//! could parse into a general uri or any of the other downcasted uri types (route, origin...)
//!

use crate::Parse;
use std::collections::{HashMap, HashSet};

mod lex;
mod semantic_tree;
mod syntax_tree;

// reflect result on itself
pub(crate) fn ref_res<T, E>(res: Result<T, E>) -> Result<E, T> {
    match res {
        Err(e) => Ok(e),
        Ok(o) => Err(o),
    }
}

use lex::{Error as LexError, Token, lex};
use semantic_tree::{
    Component, Error as SemanticError, Host, Path, Query, Scheme, User, semantic_tree,
};
use syntax_tree::{Error as SyntaxError, TokenGroup, syntax_tree};

#[derive(Debug)]
pub enum Error {
    OutputLayoutMismatch,
    ExpectedScheme,
    ExpectedUserOrHost,
    ExpectedHost,
    UnexpectedComponentAtPosition,
}

fn validate_fragment(frag: Option<Component>) -> Result<Option<Component>, Error> {
    let Some(frag) = frag else { return Ok(None) };
    match &frag {
        Component::Fragment(_) => Ok(Some(frag)),
        _ => Err(Error::UnexpectedComponentAtPosition),
    }
}

fn validate_query(query: Option<Component>) -> Result<Option<Component>, Error> {
    let Some(query) = query else { return Ok(None) };

    match &query {
        Component::Query(_) => Ok(Some(query)),
        _ => Err(Error::UnexpectedComponentAtPosition),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbsoluteUrl {
    scheme: Scheme,
    user: Option<User>,
    host: Host,
    port: u16,
    path: Path,
    query: Option<Query>,
    fragment: Option<String>,
}

impl Parse for AbsoluteUrl {
    type Token = Token;
    type TokenGroup = TokenGroup;
    type Component = Component;

    type LexError = LexError;
    type SyntaxError = SyntaxError;
    type SemanticError = SemanticError;
    type ParseError = Error;

    fn lex(s: &str) -> Result<Vec<Token>, LexError> {
        lex(s)
    }

    fn syntax_tree(tokens: Vec<Token>) -> Result<Vec<TokenGroup>, SyntaxError> {
        syntax_tree(tokens)
    }

    fn semantic_tree(groups: Vec<TokenGroup>) -> Result<Vec<Component>, SemanticError> {
        semantic_tree(groups)
    }

    fn parse(components: Vec<Component>) -> Result<Self, Error> {
        Self::from_iter(components)
    }
}

impl AbsoluteUrl {
    fn from_iter<I: IntoIterator<Item = Component>>(i: I) -> Result<Self, Error> {
        use Component::*;

        let mut iter = i.into_iter().peekable();
        let Some(Scheme(scheme)) = iter.next() else {
            return Err(Error::ExpectedScheme);
        };

        let (user, host) = match iter.next() {
            None => return Err(Error::ExpectedUserOrHost),
            Some(User(user)) => {
                let Some(Host(host)) = iter.next() else {
                    return Err(Error::ExpectedHost);
                };
                (Some(user), host)
            }
            Some(Host(host)) => (None, host),
            _ => return Err(Error::ExpectedHost),
        };

        let (port, path, query, fragment) = match iter.next() {
            None => (
                scheme.default_port(),
                semantic_tree::Path::default(),
                None,
                None,
            ),
            Some(Query(query)) => (
                scheme.default_port(),
                semantic_tree::Path::default(),
                Some(query),
                iter.next(),
            ),
            Some(Fragment(frag)) => (
                scheme.default_port(),
                semantic_tree::Path::default(),
                None,
                Some(frag),
            ),

            Some(Port(port)) => match iter.next() {
                None => (port, semantic_tree::Path::default(), None, None),
                Some(Path(path)) => (
                    port,
                    path,
                    validate_query(iter.next())?,
                    validate_fragment(iter.next())?,
                ),
                Some(Query(query)) => (
                    port,
                    semantic_tree::Path::default(),
                    query,
                    validate_fragment(iter.next())?,
                ),
                Some(Fragment(frag)) => (port, semantic_tree::Path::default(), None, frag),
                _ => Err(Error::UnexpectedComponentAtPosition),
            },
            Some(Path(path)) => match iter.next() {
                None => (scheme.default_port(), path, None, None),
                Some(Query(query)) => (
                    scheme.default_port(),
                    path,
                    query,
                    validate_fragment(iter.next())?,
                ),
                Some(Fragment(frag)) => (scheme.default_port(), path, None, frag),
                _ => return Err(Error::UnexpectedComponentAtPosition),
            },
            _ => return Err(Error::UnexpectedComponentAtPosition),
        };

        Ok(Self {
            scheme,
            user,
            host,
            port,
            path,
            query,
            fragment,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemeRelativeUrl {
    user: Option<User>,
    host: Host,
    port: u16,
    path: Path,
    query: Option<Query>,
    fragment: Option<String>,
}

impl Parse for SchemeRelativeUrl {
    type Token = Token;
    type TokenGroup = TokenGroup;
    type Component = Component;

    type LexError = LexError;
    type SyntaxError = SyntaxError;
    type SemanticError = SemanticError;
    type ParseError = Error;

    fn lex(s: &str) -> Result<Vec<Token>, LexError> {
        lex(s)
    }

    fn syntax_tree(tokens: Vec<Token>) -> Result<Vec<TokenGroup>, SyntaxError> {
        syntax_tree(tokens)
    }

    fn semantic_tree(groups: Vec<TokenGroup>) -> Result<Vec<Component>, SemanticError> {
        semantic_tree(groups)
    }

    fn parse(components: Vec<Component>) -> Result<Self, Error> {
        Self::from_iter(components)
    }
}

impl SchemeRelativeUrl {
    fn from_iter<I: IntoIterator>(i: I) -> Result<Self, Error> {
        use Component::*;

        let mut iter = i.into_iter().peekable();

        let (user, host) = match iter.next() {
            None => return Err(Error::ExpectedUserOrHost),
            Some(User(user)) => {
                let Some(Host(host)) = iter.next() else {
                    return Err(Error::ExpectedHost);
                };
                (Some(user), host)
            }
            Some(Host(host)) => (None, host),
            _ => return Err(Error::ExpectedHost),
        };

        // WARN if port is implicit
        // we assume tls 443
        // then user can fix it using SchemeRelative.align_port(Scheme / port number)
        let (port, path, query, fragment) = match iter.next() {
            None => (443, semantic_tree::Path::default(), None, None),
            Some(Query(query)) => (
                443,
                semantic_tree::Path::default(),
                Some(query),
                iter.next(),
            ),
            Some(Fragment(frag)) => (443, semantic_tree::Path::default(), None, Some(frag)),

            Some(Port(port)) => match iter.next() {
                None => (port, semantic_tree::Path::default(), None, None),
                Some(Path(path)) => match iter.next() {
                    None => (port, path, None, None),
                    Some(Query(query)) => (port, path, query, validate_fragment(iter.next())?),
                    Some(Fragment(frag)) => (port, path, None, frag),
                    _ => return Err(Error::UnexpectedComponentAtPosition),
                },
                Some(Query(query)) => (
                    port,
                    semantic_tree::Path::default(),
                    query,
                    validate_fragment(iter.next())?,
                ),
                Some(Fragment(frag)) => (port, semantic_tree::Path::default(), None, frag),
                _ => Err(Error::UnexpectedComponentAtPosition),
            },
            Some(Path(path)) => match iter.next() {
                None => (443, path, None, None),
                Some(Query(query)) => (443, path, query, validate_fragment(iter.next())?),
                Some(Fragment(frag)) => (443, path, None, frag),
                _ => return Err(Error::UnexpectedComponentAtPosition),
            },
            _ => return Err(Error::UnexpectedComponentAtPosition),
        };

        Ok(Self {
            user,
            host,
            port,
            path,
            query,
            fragment,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathRelativeUrl {
    path: Path,
    query: Option<Query>,
    fragment: Option<String>,
}

impl Parse for PathRelativeUrl {
    type Token = Token;
    type TokenGroup = TokenGroup;
    type Component = Component;

    type LexError = LexError;
    type SyntaxError = SyntaxError;
    type SemanticError = SemanticError;
    type ParseError = Error;

    fn lex(s: &str) -> Result<Vec<Token>, LexError> {
        lex(s)
    }

    fn syntax_tree(tokens: Vec<Token>) -> Result<Vec<TokenGroup>, SyntaxError> {
        syntax_tree(tokens)
    }

    fn semantic_tree(groups: Vec<TokenGroup>) -> Result<Vec<Component>, SemanticError> {
        semantic_tree(groups)
    }

    fn parse(components: Vec<Component>) -> Result<Self, Error> {
        Self::from_iter(components)
    }
}

impl PathRelativeUrl {
    fn from_iter<I: IntoIterator>(i: I) -> Result<Self, Error> {
        use Component::*;

        let mut iter = i.into_iter().peekable();

        let (path, query, fragment) = match iter.next() {
            None => (semantic_tree::Path::default(), None, None),
            Some(Query(query)) => (semantic_tree::Path::default(), Some(query), iter.next()),
            Some(Fragment(frag)) => (semantic_tree::Path::default(), None, Some(frag)),
            Some(Path(path)) => match iter.next() {
                None => (path, None, None),
                Some(Query(query)) => (path, query, validate_fragment(iter.next())?),
                Some(Fragment(frag)) => (path, None, frag),
                _ => return Err(Error::UnexpectedComponentAtPosition),
            },

            _ => return Err(Error::UnexpectedComponentAtPosition),
        };

        Ok(Self {
            path,
            query,
            fragment,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Url {
    Absolute(AbsoluteUrl),
    SchemeRelative(SchemeRelativeUrl),
    PathRelative(PathRelativeUrl),
}

impl Parse for Url {
    type Token = Token;
    type TokenGroup = TokenGroup;
    type Component = Component;

    type LexError = LexError;
    type SyntaxError = SyntaxError;
    type SemanticError = SemanticError;
    type ParseError = Error;

    fn lex(s: &str) -> Result<Vec<Token>, LexError> {
        lex(s)
    }

    fn syntax_tree(tokens: Vec<Token>) -> Result<Vec<TokenGroup>, SyntaxError> {
        syntax_tree(tokens)
    }

    fn semantic_tree(groups: Vec<TokenGroup>) -> Result<Vec<Component>, SemanticError> {
        semantic_tree(groups)
    }

    fn parse(components: Vec<Component>) -> Result<Self, Error> {
        if components[0].is_scheme() {
            AbsoluteUrl::parse(components).map(Self::Absolute)
        } else if components[0].is_path() {
            PathRelativeUrl::parse(components).map(Self::PathRelative)
        } else {
            SchemeRelativeUrl::parse(components).map(Self::SchemeRelative)
        }
    }
}

impl AbsoluteUrl {
    fn update_scheme(&mut self, scheme: Scheme) {
        self.scheme = scheme;
    }

    fn update_host(&mut self, host: Vec<String>) {
        self.host = Host::new(host.into_iter());
    }

    fn update_port(&mut self, port: u16) {
        self.port = port;
    }

    fn update_path(&mut self, path: Vec<String>) {
        self.path = Path::new(path.into_iter());
    }

    fn update_query(&mut self, query: Query) {
        self.query = Some(query);
    }

    fn update_fragment(&mut self, frag: String) {
        self.frag = Some(frag);
    }
}

impl AbsoluteUrl {
    #[deprecated(note = "use Wildcardish<Origin> api instead")]
    pub fn matches_any_origin(&self) -> bool {
        self.host.len() == 1 && self.host[0] == "*"
    }
}

impl std::str::FromStr for Url {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(parser) = Parse::new(s) else {
            return Err(ref_res(Error::url(0)).unwrap());
        };

        parser.parse()
    }
}

impl Url {
    pub fn path_absolute(
        path: Vec<&str>,
        query: Option<(HashMap<&str, &str>, HashSet<&str>)>,
        fragment: Option<String>,
    ) -> Self {
        Self {
            path: Some(path.into_iter().map(|s| s.into()).collect()),
            query: query.map(|(params, attrs)| Query::from_colls(params, attrs)),
            fragment,
            ..Default::default()
        }
    }

    pub fn scheme_relative(
        domain: Vec<String>,
        port: Option<u16>,
        path: Option<Vec<String>>,
        query: Option<(HashMap<&str, &str>, HashSet<&str>)>,
        fragment: Option<String>,
    ) -> Self {
        Self {
            domain: Some(domain),
            port,
            path,
            query: query.map(|(params, attrs)| Query::from_colls(params, attrs)),
            fragment,
            ..Default::default()
        }
    }

    pub fn new(s: &str) -> Result<Self, Error> {
        s.parse()
    }

    pub fn from_parts(
        scheme: Option<Scheme>,
        domain: Option<Vec<String>>,
        port: Option<u16>,
        path: Option<Vec<String>>,
        query: Option<Query>,
        fragment: Option<String>,
    ) -> Self {
        Self {
            domain,
            port,
            path,
            query,
            fragment,
            scheme,
        }
    }

    pub fn absolute(
        scheme: Scheme,
        domain: Vec<String>,
        port: Option<u16>,
        path: Option<Vec<String>>,
        query: Option<Query>,
        fragment: Option<String>,
    ) -> Self {
        Self {
            domain: Some(domain),
            port,
            path,
            query,
            fragment,
            scheme: Some(scheme),
        }
    }

    pub fn sequence(&self) -> String {
        let scheme = self
            .scheme
            .map(|s| format!("{}://", s.as_str()))
            .unwrap_or_default();

        let mut domain = if let Some(ref domain) = self.domain {
            let mut domain = domain.into_iter().fold(scheme, |acc, d| acc + d + ".");
            domain.pop();

            domain
        } else {
            scheme
        };

        if let Some(port) = self.port {
            domain.push_str(&format!(":{}", port));
        }

        let mut path = if let Some(ref path) = self.path {
            if path.is_empty() {
                "/".to_owned()
            } else {
                path.into_iter().fold(domain, |acc, s| acc + "/" + s)
            }
        } else {
            domain
        };

        if let Some(ref query) = self.query.as_ref().map(|q| q.sequence()) {
            path.push_str(query);
        }

        if let Some(ref fragment) = self.fragment {
            path.push('#');
            path.push_str(fragment);
        }

        path
    }

    /// downcasts the Url instance to sub url type
    pub fn interpret<T>(self) -> Result<T, Error>
    where
        T: TryFrom<Self, Error = Error>,
    {
        self.try_into()
    }
}

impl Url {
    pub fn scheme(&self) -> Option<Scheme> {
        self.scheme
    }

    pub fn take_domain(&mut self) -> Option<Vec<String>> {
        let Some(ref mut domain) = self.domain else {
            return None;
        };

        Some(std::mem::take(domain))
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    pub fn take_path(&mut self) -> Option<Vec<String>> {
        let Some(ref mut path) = self.path else {
            return None;
        };

        Some(std::mem::take(path))
    }

    pub fn take_query(&mut self) -> Option<Query> {
        let Some(ref mut query) = self.query else {
            return None;
        };

        Some(std::mem::take(query))
    }

    pub fn take_fragment(&mut self) -> Option<String> {
        let Some(ref mut fragment) = self.fragment else {
            return None;
        };

        Some(std::mem::take(fragment))
    }
}
