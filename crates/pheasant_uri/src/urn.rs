//! module for the urn type

use crate::{Parse, error_inheritance, from_str};

mod lex {
    use crate::token_is;

    #[derive(Debug)]
    pub enum Error {
        ExpectedUrnScheme,
        ExpectedUrnTokenFoundEoT,
        ExpectedNidTokenFoundEoT,
        ExpectedNssTokenFoundEoT,
    }

    #[derive(Debug)]
    pub enum Token {
        Urn(String),
        Nid(String),
        Nss(String),
    }

    impl Token {
        token_is!(is_nid, Nid());
    }

    pub fn lex(s: &str) -> Result<Vec<Token>, Error> {
        let mut tokens = s.splitn(3, ':');
        let Some(urn) = tokens.next() else {
            return Err(Error::ExpectedUrnTokenFoundEoT);
        };
        if urn.to_lowercase() != "urn" {
            return Err(Error::ExpectedUrnScheme);
        }

        let Some(nid) = tokens.next().map(|nid| Token::Nid(nid.to_owned())) else {
            return Err(Error::ExpectedNidTokenFoundEoT);
        };

        let Some(nss) = tokens.next().map(|nss| Token::Nss(nss.to_owned())) else {
            return Err(Error::ExpectedNssTokenFoundEoT);
        };

        Ok(vec![nid, nss])
    }
}

mod semantic_tree {
    use super::{Nid, Nss, lex::Token};
    use crate::{SUB_DELIMS, SpellChecker, SpellingError, UNRESERVED};

    #[derive(Debug)]
    pub enum Error {
        ExpectedNidThenNss,
        ExpectedEoT,
    }

    pub enum Component {
        Nid(Nid),
        Nss(Nss),
    }

    pub fn semantic_tree(tokens: Vec<Token>) -> Result<Vec<Component>, Error> {
        let mut iter = tokens.into_iter();
        let (Some(Token::Nid(nid)), Some(Token::Nss(nss))) = (iter.next(), iter.next()) else {
            return Err(Error::ExpectedNidThenNss);
        };

        if iter.next().is_some() {
            return Err(Error::ExpectedEoT);
        }

        Ok(vec![Component::Nid(Nid(nid)), Component::Nss(Nss(nss))])
    }

    impl<'a> SpellChecker for &'a Nid {
        const ALLOWED: &'static [char] = &['-'];
        type Input = &'a Token;

        fn spell_check(token: &Token) -> Result<(), SpellingError> {
            let Token::Nid(nid) = token else {
                return Err(SpellingError::InvalidTokenForComponent);
            };

            if nid
                .chars()
                .any(|c| !c.is_alphanumeric() && !Self::ALLOWED.contains(&c) && !c.is_ascii())
            {
                return Err(SpellingError::InvalidCharsForComponent);
            }

            Ok(())
        }
    }

    impl<'a> SpellChecker for &'a Nss {
        const ALLOWED: &'static [char] = &[':', '@', '/'];
        type Input = &'a Token;

        fn spell_check(token: &Token) -> Result<(), SpellingError> {
            let Token::Nss(nss) = token else {
                return Err(SpellingError::InvalidTokenForComponent);
            };

            if nss.chars().any(|c| {
                !c.is_alphanumeric()
                    && !Self::ALLOWED.contains(&c)
                    && !SUB_DELIMS.contains(&c)
                    && !UNRESERVED.contains(&c)
            }) {
                return Err(SpellingError::InvalidCharsForComponent);
            }

            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct Urn {
    nid: Nid,
    nss: Nss,
}

#[derive(Debug)]
pub struct Nid(String);

#[derive(Debug)]
pub struct Nss(String);

#[derive(Debug)]
pub enum Error {
    Lex(lex::Error),
    Semantic(semantic_tree::Error),
    ExpectedNss,
    ExpectedNid,
}

error_inheritance!(Lex { lex::Error, Error });
error_inheritance!(Semantic { semantic_tree::Error, Error });

impl Parse for Urn {
    type Token = lex::Token;
    type TokenGroup = lex::Token;
    type Component = semantic_tree::Component;

    type LexError = lex::Error;
    type SyntaxError = lex::Error;
    type SemanticError = semantic_tree::Error;
    type ParseError = Error;

    fn lex(s: &str) -> Result<Vec<Self::Token>, Self::LexError> {
        lex::lex(s)
    }

    fn syntax_tree(tokens: Vec<Self::Token>) -> Result<Vec<Self::TokenGroup>, Self::SyntaxError> {
        Ok(tokens)
    }

    fn semantic_tree(
        groups: Vec<Self::TokenGroup>,
    ) -> Result<Vec<Self::Component>, Self::SemanticError> {
        semantic_tree::semantic_tree(groups)
    }

    /// final parse function call on input to do the whole parse operation
    fn parse(components: Vec<Self::Component>) -> Result<Self, Self::ParseError> {
        Self::from_iter(components)
    }
}

impl Urn {
    fn from_iter<I: IntoIterator<Item = semantic_tree::Component>>(i: I) -> Result<Self, Error> {
        use semantic_tree::Component;

        let mut iter = i.into_iter();
        let Some(Component::Nid(nid)) = iter.next() else {
            return Err(Error::ExpectedNid);
        };
        let Some(Component::Nss(nss)) = iter.next() else {
            return Err(Error::ExpectedNss);
        };

        Ok(Self { nid, nss })
    }
}

from_str!(Urn, Error);
