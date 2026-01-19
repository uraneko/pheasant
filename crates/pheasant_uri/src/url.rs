//! parse step
//! parses the components into a uri
//! could parse into a general uri or any of the other downcasted uri types (route, origin...)
//!
use crate::Parse;

pub mod derivatives;
mod lex;
mod semantic_tree;
mod syntax_tree;

/// swaps the ok and err values of a Result<T,E> returning a Result<E, T>
pub(crate) fn flip_result<T, E>(res: Result<T, E>) -> Result<E, T> {
    match res {
        Err(e) => Ok(e),
        Ok(o) => Err(o),
    }
}

use lex::{Error as LexError, Token, lex};
use semantic_tree::{Component, Error as SemanticError, User, semantic_tree};
pub use semantic_tree::{Host, Path, Query, Scheme};
use syntax_tree::{Error as SyntaxError, TokenGroup, syntax_tree};

pub trait PercentEncodable {
    const TABLE: &'static [(&'static str, &'static str)];
    type Err;

    /// checks if s contains any component forbidden chars that should be percent encoded
    fn should_encode(s: &str) -> bool {
        Self::TABLE.iter().any(|(_, ch)| s.contains(ch))
    }

    /// checks if s contains any percent encoded values
    fn is_encoded(s: &str) -> bool {
        Self::TABLE.iter().any(|(sub, _)| s.contains(sub))
    }

    /// replaces all sightings of a component forbidden char in s with its percent encodng
    /// or vice-versa
    fn replace_all(s: &mut String, sub: &str, ch: &str) {
        while s.contains(sub) {
            *s = s.replace(sub, ch);
        }
    }

    /// percent decodes a url component
    fn decode(s: &mut String) {
        if !Self::is_encoded(&s) {
            return;
        }

        Self::TABLE.iter().for_each(|(sub, ch)| {
            if s.contains(sub) {
                Self::replace_all(s, sub, ch)
            }
        });
    }

    /// percent encodes a url component
    fn encode(s: &mut String) {
        if !Self::should_encode(&s) {
            return;
        }

        Self::TABLE.iter().for_each(|(sub, ch)| {
            if s.contains(ch) {
                Self::replace_all(s, ch, sub)
            }
        });
    }

    /// applies decode to all sequence tokens of a url component
    fn decode_component(tokens: &mut Vec<Token>) {
        let mut iter = tokens.iter_mut().filter(|t| t.is_seq());
        while let Some(Token::Seq(s)) = iter.next() {
            Self::decode(s);
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Str(core::str::Utf8Error),
    Lex(LexError),
    Syntax(SyntaxError),
    Semantic(SemanticError),
    OutputLayoutMismatch,
    ExpectedScheme,
    ExpectedUserOrHost,
    ExpectedHost,
    UnexpectedComponentAtPosition,
}

fn validate_fragment(frag: Option<Component>) -> Result<Option<String>, Error> {
    let Some(frag) = frag else { return Ok(None) };
    match frag {
        Component::Fragment(frag) => Ok(Some(frag)),
        _ => Err(Error::UnexpectedComponentAtPosition),
    }
}

fn validate_query(query: Option<Component>) -> Result<Option<Query>, Error> {
    let Some(query) = query else { return Ok(None) };

    match query {
        Component::Query(query) => Ok(Some(query)),
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
                validate_fragment(iter.next())?,
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
                    Some(query),
                    validate_fragment(iter.next())?,
                ),
                Some(Fragment(frag)) => (port, semantic_tree::Path::default(), None, Some(frag)),
                _ => return Err(Error::UnexpectedComponentAtPosition),
            },
            Some(Path(path)) => match iter.next() {
                None => (scheme.default_port(), path, None, None),
                Some(Query(query)) => (
                    scheme.default_port(),
                    path,
                    Some(query),
                    validate_fragment(iter.next())?,
                ),
                Some(Fragment(frag)) => (scheme.default_port(), path, None, Some(frag)),
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
    fn from_iter<I: IntoIterator<Item = Component>>(i: I) -> Result<Self, Error> {
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
                validate_fragment(iter.next())?,
            ),
            Some(Fragment(frag)) => (443, semantic_tree::Path::default(), None, Some(frag)),

            Some(Port(port)) => match iter.next() {
                None => (port, semantic_tree::Path::default(), None, None),
                Some(Path(path)) => match iter.next() {
                    None => (port, path, None, None),
                    Some(Query(query)) => {
                        (port, path, Some(query), validate_fragment(iter.next())?)
                    }
                    Some(Fragment(frag)) => (port, path, None, Some(frag)),
                    _ => return Err(Error::UnexpectedComponentAtPosition),
                },
                Some(Query(query)) => (
                    port,
                    semantic_tree::Path::default(),
                    Some(query),
                    validate_fragment(iter.next())?,
                ),
                Some(Fragment(frag)) => (port, semantic_tree::Path::default(), None, Some(frag)),
                _ => return Err(Error::UnexpectedComponentAtPosition),
            },
            Some(Path(path)) => match iter.next() {
                None => (443, path, None, None),
                Some(Query(query)) => (443, path, Some(query), validate_fragment(iter.next())?),
                Some(Fragment(frag)) => (443, path, None, Some(frag)),
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

impl SchemeRelativeUrl {
    pub fn align_port(&mut self, port: u16) {
        self.port = port;
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
        // TODO here we decode the sequences
        semantic_tree(groups)
    }

    fn parse(components: Vec<Component>) -> Result<Self, Error> {
        Self::from_iter(components)
    }
}

impl PathRelativeUrl {
    fn from_iter<I: IntoIterator<Item = Component>>(i: I) -> Result<Self, Error> {
        use Component::*;

        let mut iter = i.into_iter().peekable();
        let (path, query, fragment) = match iter.next() {
            None => (semantic_tree::Path::default(), None, None),
            Some(Query(query)) => (
                semantic_tree::Path::default(),
                Some(query),
                validate_fragment(iter.next())?,
            ),
            Some(Fragment(frag)) => (semantic_tree::Path::default(), None, Some(frag)),
            Some(Path(path)) => match iter.next() {
                None => (path, None, None),
                Some(Query(query)) => (path, Some(query), validate_fragment(iter.next())?),
                Some(Fragment(frag)) => (path, None, Some(frag)),
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

#[macro_export]
macro_rules! error_inheritance {
    ($var: ident { $src: ty, $dest: ty }) => {
        impl From<$src> for $dest {
            fn from(err: $src) -> $dest {
                Self::$var(err)
            }
        }
    };
}

error_inheritance!(Lex { LexError, Error });
error_inheritance!(Syntax { SyntaxError, Error });
error_inheritance!(Semantic {
    SemanticError,
    Error
});

#[macro_export]
macro_rules! from_str {
    ($uri: ty, $err: ty) => {
        impl core::str::FromStr for $uri {
            type Err = $err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let tokens = Self::lex(s)?;
                let syntax_tree = Self::syntax_tree(tokens)?;
                let semantic_tree = Self::semantic_tree(syntax_tree)?;

                Self::parse(semantic_tree)
            }
        }
    };
}

from_str!(Url, Error);
from_str!(AbsoluteUrl, Error);
from_str!(SchemeRelativeUrl, Error);
from_str!(PathRelativeUrl, Error);

// impl FromStr for Url {
//     type Err = Error;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let tokens = Self::lex(s)?;
//         let syntax_tree = Self::syntax_tree(tokens)?;
//         let semantic_tree = Self::semantic_tree(syntax_tree)?;
//
//         Self::parse(semantic_tree)
//     }
// }
