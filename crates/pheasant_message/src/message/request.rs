use crate::Scrutinizer;
use pheasant_core::err_stt;
use pheasant_headers::{Cookie, RequestCors};
use pheasant_uri::{Query, Resource};

pub mod headers;
pub mod lex;
pub mod scrutinize;

pub use lex::{Token, lex};

pub struct Request {
    headers: HashMap<Vec<u8>, Vec<u8>>,
    proto: Protocol,
    method: Method,
    resource: Resource,
    query: Option<Query>,
    cors: Option<RequestCors>,
    cookies: Option<HashSet<Cookie>>,
    body: Option<Vec<u8>>,
}

pub struct Builder {
    headers: Option<HashMap<Vec<u8>, Vec<u8>>>,
    proto: Protocol,
    method: Method,
    resource: Route,
    body: Option<Vec<u8>>,
}

impl Builder {
    fn header(&mut self, h: Vec<u8>, f: Vec<u8>) {
        let Some(headers) = self.headers else {
            self.headers = Some(HashMap::from([(h, f)]));
            return;
        };

        headers.insert(h, f);
    }

    // currently not in use
    // fn headers<I: Iterator<Item = (Vec<u8>, Vec<u8>)>>(&mut self, iter: I) {
    //     let Some(headers) = self.headers else {
    //         self.headers = Some(HashMap::from_iter(iter));
    //         return;
    //     };
    //
    //     headers.extend(iter);
    // }

    fn body(&mut self, body: Option<Vec<u8>>) {
        self.body = body;
    }

    fn build(self) -> Request {
        let cors = RequestCors::from_headers(&mut self.headers);
        let cookies = <HashSet<Cookie>>::from_headers(&mut self.headers);
        let [resource, query] = self.resource.into();

        Request {
            query,
            headers,
            cookies,
            cors,
            proto: self.proto,
            method: self.method,
            resource: self.resource,
            body: self.body,
        }
    }
}

impl Request {
    pub fn parse(tokens: Vec<Token>) -> Result<Self, ErrorStatus> {
        let body = match tokens.last() {
            Token::Body(b) => tokens.pop(),
            _ => None,
        };

        let mut iter = tokens.into_iter();
        let [
            Some(Token::Method(method)),
            Some(Token::Uri(resource)),
            Some(Token::Proto(proto)),
        ] = [iter.next(), iter.next(), iter.next()]
        else {
            return Err(err_stt!(BadRequest));
        };

        let mut builder = Request::builder(method, resource, proto);
        let body = body.map(|b| {
            let Token::Body(body) = b else {
                unreachable!("we know tokens end with body")
            };

            b
        });
        builder.body(body);

        while let Some(Token::Header(header)) = iter.next() {
            let Some(Token::Field(field)) = iter.next() else {
                return Err(LexError::TokenMismatch("expected field after header token"));
            };
            builder.header(header, field)
        }

        Ok(builder.build())
    }

    // F: scrutinizer is a function that takes req and whatever else is necessary
    // generates the scrutinizing types
    // and then runs their Type::scrutunize()?
    // if no error is returned by the end then request is good
    // else if error we move to Message::Error variant from Message::Request
    fn scrutinize<F, S: Scrutinizer>(&self, scrutinizer: F) -> Result<(), ErrorStatus>
    where
        F: Fn(&Request, SocketRef<'_>) -> Result<(), ErrorStatus>,
    {
        todo!()
    }
}
