pub mod url;

pub use url::{AbsoluteUrl, PathRelativeUrl, SchemeRelativeUrl, Url};

// pub use components::{Host, Nid, Nss, Path, Query, Scheme, User};
// pub use parse::{Blob, Data, File, Host, Javascript, Origin, Resource, Route, Uri, Url, Urn};

// WARN We assume Http/Https schemes only
// we also assume no user nor fragment components

// this is for individual components
// e.g., url's Scheme, Path or urn's NSS...
pub trait Sanitizer {
    const FORBIDDEN: &'static [&'static str];
    type Err;

    fn sanitize(&self) -> Result<(), Self::Err>;
}

struct PercentEncodedChar(char);

// this is for individual components
// e.g., url's Scheme, Path or urn's NSS...
pub trait PercentEncoded {
    const TABLE: &'static [(PercentEncodedChar, char)];
    type Err;

    fn encode(s: &str) -> Result<String, Self::Err>;

    fn decode(s: &str) -> Result<String, Self::Err>;
}

pub const SUB_DELIMS: [char; 11] = ['!', '$', '&', '\'', '(', ')', '*', '+', ',', ';', '='];
pub const GEN_DELIMS: [char; 7] = [':', '/', '?', '#', '[', ']', '@'];
pub const UNRESERVED: [char; 4] = ['.', '-', '_', '~']; // + alphanumeric

// this is for individual components
// e.g., url's Scheme, Path or urn's NSS...
pub trait SpellChecker {
    const ALLOWED: &'static [char];
    type Input;

    fn spell_check(group: Self::Input) -> Result<(), SpellingError>;
}

pub enum SpellingError {
    InvalidCharsForComponent,
    InvalidTokenForComponent,
}

// this is for uri sub-types
// e.g., url, urn, file, data, blob, javascript...
pub trait Parse: Sized {
    type Token;
    type TokenGroup;
    type Component;

    type LexError: Into<Self::ParseError>;
    type SyntaxError: Into<Self::ParseError>;
    type SemanticError: Into<Self::ParseError>;
    type ParseError: Into<Self::ParseError>;

    fn lex(s: &str) -> Result<Vec<Self::Token>, Self::LexError>;

    fn syntax_tree(tokens: Vec<Self::Token>) -> Result<Vec<Self::TokenGroup>, Self::SyntaxError>;

    fn semantic_tree(
        groups: Vec<Self::TokenGroup>,
    ) -> Result<Vec<Self::Component>, Self::SemanticError>;

    /// final parse function call on input to do the whole parse operation
    fn parse(tree: Vec<Self::Component>) -> Result<Self, Self::ParseError>;

    // / after getting scheme from the tokens
    // / we can then deduce the type of the uri
    // / and use Parse methods on  the tokens iterator
    // fn evaluate(&self, tokens: &mut impl Iterator<Item = Token>);
}

// Define the entities
// input string = string repr of the uri
// components   = the components of the uri in data repr
// uris         = the data repr of uris
// tokens       = the simplest fragmented repr of a uri
// token groups = each token group represents a component
// units        = basically chars, each unit is 1 unicode char
//
// Defines the relations between entities
// input string <-> components
// - input can be broken into tokens
// - tokens can be classified into token groups
// - token groups can be parsed into components
// - components can be combined/interpreted into uris
// - uris can be serialize into (input) strings
// * everything can be broken down into chars
//
// Define the layers of operations
// the data layer        = contains user facing/useful data
// the pseudo-data layer = contains pseudo data that exists temporarly when relationship logic is being carried out
// the unit layer        = everything is broken into chars here
//
// TODO socket.translate_sematic_requests: bool
// TODO enum ResourceQuery { KeyVal(&'static str), MaybeKeyVal(Option<&'static str>), Attr(&'static str), }
