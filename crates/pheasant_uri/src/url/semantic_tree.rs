//! semantic analysis
//! parses the components into their values
//! also validates the tokens contents

// TODO need to parse out the actual components here
// error out if the components are ill formed or contain malicious contents using
// SpellChecker and Sanitizer

use super::{lex::Token, syntax_tree::TokenGroup};
use crate::{PercentEncoded, SpellChecker};

mod host;
mod path;
mod query;
mod scheme;
mod user;
// URL -> http|https :// user @ host : port / path ? query # fragment

pub use host::Host;
pub use path::Path;
pub use query::Query;
pub use scheme::Scheme;
pub use user::User;
// TODO on http1.1 Host header must always be set once.

#[derive(Debug)]
pub enum Error {
    ComponentTokensMismatch,
    ExpectedSequence,
    SchemeNotRecognized,
}

#[derive(Debug)]
pub enum Component {
    Scheme(Scheme),
    User(User),
    Host(Host),
    Port(u16),
    Path(Path),
    Query(Query),
    Fragment(String),
}

macro_rules! comp_is {
    ($name: ident, $var: ident) => {
        pub fn $name(&self) -> bool {
            let Self::$var(_) = self else { return false };

            true
        }
    };
}

impl Component {
    comp_is!(is_port, Port);
    comp_is!(is_scheme, Scheme);
    comp_is!(is_path, Path);
}

// BUG query parses to nothing
// BUG path parsing ignores the str values of non-seq tokens
// e.g., `index.html` parses to [`index`, `html`] <- as if it was `index/html`
pub fn semantic_tree(groups: Vec<TokenGroup>) -> SemanticResult<Vec<Component>> {
    Ok(groups
        .into_iter()
        .map(|g| {
            Ok::<Component, Error>(match g {
                TokenGroup::Scheme(tokens) => Component::Scheme(Scheme::try_from(tokens)?),
                TokenGroup::User(tokens) => Component::User(User::try_from(tokens)?),
                TokenGroup::Host(tokens) => Component::Host(Host::try_from(tokens)?),
                TokenGroup::Port(token) => Component::Port(u16::try_from(token)?),
                TokenGroup::Path(tokens) => Component::Path(Path::try_from(tokens)?),
                TokenGroup::Query(tokens) => Component::Query(Query::try_from(tokens)?),
                TokenGroup::Fragment(tokens) => {
                    Component::Fragment(String::try_from(Tokens(tokens))?)
                }
            })
        })
        .flatten()
        .collect())
}

// TODO everything below should be deprecated
type SemanticResult<T> = Result<T, Error>;

impl TryFrom<Vec<Token>> for Scheme {
    type Error = Error;

    fn try_from(mut tokens: Vec<Token>) -> SemanticResult<Self> {
        if tokens.len() == 1 {
            let Some(Token::Seq(seq)) = tokens.pop() else {
                return Err(Error::ExpectedSequence);
            };

            return Ok(seq.parse::<Self>().unwrap());
        }

        Err(Error::SchemeNotRecognized)
        // Scheme::Custom(
        //     tokens
        //         .into_iter()
        //         .fold("".to_owned(), |acc, t| acc + t.as_str()),
        // )
    }
}

impl TryFrom<Vec<Token>> for User {
    type Error = Error;

    fn try_from(tokens: Vec<Token>) -> SemanticResult<Self> {
        let [user, password] = {
            let mut iter = tokens.into_iter();
            let Token::Seq(user) = iter.next().unwrap() else {
                return Err(Error::ComponentTokensMismatch);
            };
            _ = iter.next();
            let Token::Seq(password) = iter.next().unwrap() else {
                return Err(Error::ComponentTokensMismatch);
            };

            [user, password]
        };

        Ok(User::new(user, password))
    }
}

impl TryFrom<Vec<Token>> for Host {
    type Error = Error;

    fn try_from(tokens: Vec<Token>) -> SemanticResult<Self> {
        Ok(Host::from_iter(tokens))
    }
}

impl TryFrom<Token> for u16 {
    type Error = Error;

    fn try_from(token: Token) -> SemanticResult<Self> {
        let Token::Seq(port) = token else {
            return Err(Error::ComponentTokensMismatch);
        };

        Ok(port.parse::<u16>().unwrap())
    }
}

impl TryFrom<Vec<Token>> for Path {
    type Error = Error;

    fn try_from(tokens: Vec<Token>) -> SemanticResult<Self> {
        Ok(Path::from_iter(tokens))
    }
}

impl TryFrom<Vec<Token>> for Query {
    type Error = Error;

    fn try_from(tokens: Vec<Token>) -> SemanticResult<Self> {
        Ok(Query::from_iter(tokens))
    }
}

#[derive(Debug, Default)]
struct Tokens(Vec<Token>);

impl TryFrom<Tokens> for String {
    type Error = Error;

    fn try_from(tokens: Tokens) -> SemanticResult<Self> {
        let tokens = tokens.0;

        Ok(query::fragment_from_iter(tokens))
    }
}
