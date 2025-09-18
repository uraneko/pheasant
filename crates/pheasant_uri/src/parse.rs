//! parse step
//! parses the components into a uri
//! could parse into a general uri or any of the other downcasted uri types (route, origin...)
//!

use crate::lex::Token as LexToken;

pub trait Parse {
    /// final parse function call on input to do the whole parse operation
    fn parse(s: &str) -> Self;

    /// after getting scheme from the tokens
    /// we can then deduce the type of the uri
    /// and use Parse methods on  the tokens iterator
    fn evaluate(&self, tokens: &mut impl Iterator<Item = LexToken>);
}
