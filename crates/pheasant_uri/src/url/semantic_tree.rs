//! semantic analysis
//! parses the components into their values
//! also validates the tokens contents

// TODO need to parse out the actual components here
// error out if the components are ill formed or contain malicious contents using
// SpellChecker and Sanitizer

use super::{lex::Token, syntax_tree::TokenGroup};
use crate::{SpellChecker, SpellingError, error_inheritance};

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

error_inheritance!(Spelling {
    SpellingError,
    Error
});

#[derive(Debug)]
pub enum Error {
    Spelling(SpellingError),
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

pub fn semantic_tree(groups: Vec<TokenGroup>) -> SemanticResult<Vec<Component>> {
    groups
        .into_iter()
        .map(|g| {
            Ok(match g {
                TokenGroup::Scheme(tokens) => {
                    Scheme::try_from(tokens).map(|scheme| Component::Scheme(scheme))?
                }
                TokenGroup::User(tokens) => {
                    User::try_from(tokens).map(|user| Component::User(user))?
                }
                TokenGroup::Host(tokens) => {
                    Host::try_from(tokens).map(|host| Component::Host(host))?
                }
                TokenGroup::Port(token) => {
                    u16::try_from(token).map(|port| Component::Port(port))?
                }
                TokenGroup::Path(tokens) => {
                    Path::try_from(tokens).map(|path| Component::Path(path))?
                }
                TokenGroup::Query(tokens) => {
                    Query::try_from(tokens).map(|query| Component::Query(query))?
                }
                TokenGroup::Fragment(tokens) => String::try_from(Fragment(tokens))
                    .map(|fragment| Component::Fragment(fragment))?,
            })
        })
        .collect()
}

// TODO everything below should be deprecated
type SemanticResult<T> = Result<T, Error>;

impl TryFrom<Vec<Token>> for Scheme {
    type Error = Error;

    fn try_from(mut tokens: Vec<Token>) -> SemanticResult<Self> {
        Scheme::spell_check(&tokens)?;

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
        User::spell_check(&tokens)?;

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
        Host::spell_check(&tokens)?;

        Ok(Host::from_iter(tokens))
    }
}

impl TryFrom<Token> for u16 {
    type Error = Error;

    fn try_from(token: Token) -> SemanticResult<Self> {
        u16::spell_check(&token)?;

        let Token::Seq(port) = token else {
            return Err(Error::ComponentTokensMismatch);
        };

        Ok(port.parse::<u16>().unwrap())
    }
}

impl TryFrom<Vec<Token>> for Path {
    type Error = Error;

    fn try_from(tokens: Vec<Token>) -> SemanticResult<Self> {
        Path::spell_check(&tokens)?;

        Ok(Path::from_iter(tokens))
    }
}

impl TryFrom<Vec<Token>> for Query {
    type Error = Error;

    fn try_from(tokens: Vec<Token>) -> SemanticResult<Self> {
        Query::spell_check(&tokens)?;

        Ok(Query::from_iter(tokens))
    }
}

#[derive(Debug, Default)]
struct Fragment(Vec<Token>);

impl TryFrom<Fragment> for String {
    type Error = Error;

    fn try_from(tokens: Fragment) -> SemanticResult<Self> {
        let tokens = tokens.0;
        String::spell_check(&tokens)?;

        Ok(query::fragment_from_iter(tokens))
    }
}
