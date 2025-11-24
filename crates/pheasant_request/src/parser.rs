pub trait Parser {
    type Error;

    // blindly returns the next token
    // figures out the token type itself
    // also returns an enum variant of the token type
    fn next(&mut self) -> Option<(&str, &[u8])>;

    // checks if next token is the request method
    // if true -> consumes data and returns token
    // else -> returns lex error without consuming
    // also returns an error if the token doesnt parse correctly into its token type
    fn method(&mut self) -> Result<&[u8], Self::Error>;

    // checks if next token is the request uri
    // if true -> consumes data and returns token
    // else -> returns lex error without consuming
    fn uri(&mut self) -> Result<&[u8], Self::Error>;

    // checks if next token is the request protocol
    // if true -> consumes data and returns token
    // else -> returns lex error without consuming
    fn proto(&mut self) -> Result<&[u8], Self::Error>;

    // checks if next token is a request header
    // if true -> consumes data and returns token
    // else -> returns lex error without consuming
    fn header(&mut self) -> Result<&[u8], Self::Error>;

    // checks if next token is a request header field
    // if true -> consumes data and returns token
    // else -> returns lex error without consuming
    fn field(&mut self) -> Result<&[u8], Self::Error>;

    // checks if next token is a request header
    // if true -> consumes data until it
    // encounters end of request headers and returns all header + field tokens
    // else -> returns lex error without consuming
    fn headers(&mut self) -> Result<&[&[u8]], Self::Error>;

    // checks if next token is the request body
    // if true -> consumes data and return token
    // else -> returns lex error without consuming
    fn body(&mut self) -> Result<&[u8], Self::Error>;
}

pub enum Error {}

pub struct Http11 {}

impl Parser for Http11 {
    type Error = Error;
}

// Request { Lookup { Forward { Query { Update { Write { Format { Send { Respond }}}}}}}}
