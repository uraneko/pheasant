//! semantic analysis
//! parses the components into their values
//! also validates the tokens contents

// TODO need to parse out the actual components here
// error out if the components are ill formed or contain malicious contents using
// SpellChecker and Sanitizer

use super::{lex::Token, syntax_tree::TokenGroup};
use crate::components::*;

#[derive(Debug)]
pub enum SemanticError {
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

pub fn semantic_tree(groups: Vec<TokenGroup>) -> SemanticResult<Vec<Component>> {
    Ok(groups
        .into_iter()
        .map(|g| match g {
            TokenGroup::Scheme(tokens) => Component::Scheme(Scheme::try_from(tokens).unwrap()),
            TokenGroup::User(tokens) => Component::User(User::try_from(tokens).unwrap()),
            TokenGroup::Host(tokens) => Component::Host(Host::try_from(tokens).unwrap()),
            TokenGroup::Port(token) => Component::Port(u16::try_from(token).unwrap()),
            TokenGroup::Path(tokens) => Component::Path(Path::try_from(tokens).unwrap()),
            TokenGroup::Query(tokens) => Component::Query(Query::try_from(tokens).unwrap()),
            TokenGroup::Fragment(tokens) => {
                Component::Fragment(String::try_from(Tokens(tokens)).unwrap())
            }
        })
        .collect())
}

type SemanticResult<T> = Result<T, SemanticError>;

impl TryFrom<Vec<Token>> for Scheme {
    type Error = SemanticError;

    fn try_from(mut tokens: Vec<Token>) -> SemanticResult<Self> {
        if tokens.len() == 1 {
            let Some(Token::Seq(seq)) = tokens.pop() else {
                return Err(SemanticError::ExpectedSequence);
            };

            return Ok(seq.parse::<Self>().unwrap());
        }

        Err(SemanticError::SchemeNotRecognized)
        // Scheme::Custom(
        //     tokens
        //         .into_iter()
        //         .fold("".to_owned(), |acc, t| acc + t.as_str()),
        // )
    }
}

impl TryFrom<Vec<Token>> for User {
    type Error = SemanticError;

    fn try_from(tokens: Vec<Token>) -> SemanticResult<Self> {
        let [user, password] = {
            let mut iter = tokens.into_iter();
            let Token::Seq(user) = iter.next().unwrap() else {
                return Err(SemanticError::ComponentTokensMismatch);
            };
            _ = iter.next();
            let Token::Seq(password) = iter.next().unwrap() else {
                return Err(SemanticError::ComponentTokensMismatch);
            };

            [user, password]
        };

        Ok(Self::new(user, password))
    }
}

impl TryFrom<Vec<Token>> for Host {
    type Error = SemanticError;

    fn try_from(tokens: Vec<Token>) -> SemanticResult<Self> {
        Ok(Host::new(tokens.into_iter().filter_map(|t| t.seq_str())))
    }
}

impl TryFrom<Token> for u16 {
    type Error = SemanticError;

    fn try_from(token: Token) -> SemanticResult<Self> {
        let Token::Seq(port) = token else {
            return Err(SemanticError::ComponentTokensMismatch);
        };

        Ok(port.parse::<u16>().unwrap())
    }
}

impl TryFrom<Vec<Token>> for Path {
    type Error = SemanticError;

    fn try_from(tokens: Vec<Token>) -> SemanticResult<Self> {
        Ok(Path::new(tokens.into_iter().filter_map(|t| t.seq_str())))
    }
}

impl TryFrom<Vec<Token>> for Query {
    type Error = SemanticError;

    fn try_from(tokens: Vec<Token>) -> SemanticResult<Self> {
        Ok(Query::new(tokens.into_iter().filter_map(|t| t.seq_str())))
    }
}

#[derive(Debug, Default)]
struct Tokens(Vec<Token>);

impl TryFrom<Tokens> for String {
    type Error = SemanticError;

    fn try_from(tokens: Tokens) -> SemanticResult<Self> {
        let tokens = tokens.0;
        todo!()
    }
}
