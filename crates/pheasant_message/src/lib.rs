extern crate alloc;
extern crate std;
use hashbrown::HashMap;
use pheasant_core::{Method, Protocol};
use pheasant_uri::Scheme;
use std::io::{Read, Write};

pub mod failure;
pub mod request;
pub mod request_headers;
pub mod requests;
pub mod response;
pub mod response_headers;
pub(crate) mod response_utils;
pub mod service;

pub use failure::Failure;
pub use requests::Request;
pub use response::Response;
pub use response_utils::{FindService, TakeRequest};
pub use service::Service;

/// validates that the read request's various parts are valid
/// e.g., Pragma: ... header + Http1.1 protocol is an error
///
/// scrutinize a request's contents
pub trait Scrutinizer {
    fn scrutinize_http1_1(&self) -> Result<(), HttpError>;

    fn scrutinize_http2(&self) -> Result<(), HttpError>;
}

struct ScrutinizeProtoHeaders<'a>
// where I: Iterator<Item = (&'a str, &'a str)>
{
    headers: &'a HashMap<String, String>,
    proto: Protocol,
}

struct ScrutinizeMethodHeaders<'a> {
    headers: &'a HashMap<String, String>,
    method: Method,
}

struct ScrutinizeSchemeProto {
    scheme: Scheme,
    proto: Protocol,
}

impl<'a> Scrutinizer for ScrutinizeProtoHeaders<'a> {}

impl Request {
    fn scrutinize<F, S: Scrutinizer>(&self, scrutinizer: F) -> Result<(), HttpError>
    where
        F: Fn(S) -> Result<(), HttpError>,
    {
        todo!()
    }
}

struct Respond {}

impl Respond {
    fn initialize(req: Request, params: &ResourceParams) -> Self {}

    fn insert_status_headers(&mut self) {}
}

trait Requester: Read {
    fn read_req(&mut self) -> Request;
}

trait Respondent: Write {
    fn write_res(&mut self, resp: Respond) -> Result<(), ()>;

    fn write_err(&mut self, err: HttpError) -> Result<(), ()>;
}

impl Message {
    fn request<R: Requester>(read: &mut R) -> Self {
        Self::Request(read.read_req())
    }

    fn respond(self, params: &ResourceParams) -> Self {
        let Self::Request(req) = self else { panic!() };

        let resp_init = Respond::initialize(req, params);
        Self::Respond(resp_init)
    }

    fn is_err(&self) -> bool {
        match self {
            Self::ErrorOut { .. } => true,
            _ => false,
        }
    }
}

pub enum Message {
    Request(Request),
    Respond(Respond),
    Forward(Forward),
    Preflight(Preflight),
    ErrorOut(HttpError),
}

struct HttpError {
    resource: String,
    method: Method,
    proto: Protocol,
    status: u16,
}
