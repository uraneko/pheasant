use crate::Request;
use hashbrown::{HashMap, HashSet};
use mime::Mime;
use pheasant_core::{Protocol, Status};
use pheasant_headers::{Cookie, ResponseCors};

pub struct Respond<'a> {
    proto: Protocol,
    status: Status,
    body: Option<Vec<u8>>,
    headers: HashMap<Vec<u8>, Vec<u8>>,
    cookies: Option<HashSet<Cookie>>,
    cors: Option<ResponseCors<'a>>,
}

impl<'a> Respond<'a> {
    pub fn builder() -> Builder<'a> {
        Builder::default()
    }
}

#[derive(Debug, Default)]
pub struct Builder<'a> {
    status: Option<Status>,
    body: Option<Vec<u8>>,
    headers: Option<HashMap<Vec<u8>, Vec<u8>>>,
    cookies: Option<HashSet<Cookie>>,
    cors: Option<ResponseCors<'a>>,
}

impl<'a> Builder<'a> {
    pub fn status(mut self, status: impl Into<Status>) -> Self {
        self.status = Some(status.into());
    }
    pub fn build(self) -> Respond<'a> {
        todo!()
    }

    // checks that status and mime are set before building respond
    fn check_basic_fields() {}

    // checks that cors field is set when we handle a cross origin request
    fn check_cors_field() {}
}

pub struct ResourceParams<'a> {
    s: &'a str,
}

impl<'a> Respond<'a> {
    pub fn initialize<'b>(req: Request, params: ResourceParams<'b>) -> Self
    where
        'a: 'b,
    {
    }

    pub fn insert_status_headers(&mut self) {}
}
