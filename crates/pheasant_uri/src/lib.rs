pub mod components;
pub mod encoding;
pub mod parse;
pub mod parsing;
pub mod spell_checker;

pub use components::{Component, Host, Path, Query, Scheme};
pub use encoding::PercentEncoded;
pub use parse::Parse;
pub use parsing::{lex, semantic_tree, syntax_tree};
pub use spell_checker::SpellChecker;
// pub use components::{Host, Nid, Nss, Path, Query, Scheme, User};
// pub use parse::{Blob, Data, File, Host, Javascript, Origin, Resource, Route, Uri, Url, Urn};

// WARN We assume Http/Https schemes only
// we also assume no user nor fragment components

pub trait Sanitizer {}

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
