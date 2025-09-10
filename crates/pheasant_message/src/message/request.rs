use crate::Scrutinizer;

pub mod headers;
pub mod lex;

pub use lex::{Token, lex};

pub struct Request {
    headers: HashMap<String, String>,
    proto: Protocol,
    method: Method,
    resource: String,
    query: Option<Query>,
    cors: Option<RequestCors>,
    cookies: Option<HashSet<Cookie>>,
}

impl Request {
    fn parse(tokens: Vec<Token>) -> Result<Self, HttpError> {}
}

impl Request {
    // F: scrutinizer is a function that takes req and whatever else is necessary
    // generates the scrutinizing types
    // and then runs their Type::scrutunize()?
    // if no error is returned by the end then request is good
    // else if error we move to Message::Error variant from Message::Request
    fn scrutinize<F, S: Scrutinizer>(&self, scrutinizer: F) -> Result<(), HttpError>
    where
        F: Fn(S) -> Result<(), HttpError>,
    {
        todo!()
    }
}
