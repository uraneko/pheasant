use hashbrown::{HashMap, HashSet};
use mime::Mime;
use pheasant_core::Protocol;
use pheasant_headers::{Cookie, ResponseCors};

pub struct Respond {
    proto: Protocol,
    status: Status,
    body: Option<Vec<u8>>,
    headers: HashMap<Vec<u8>, Vec<u8>>,
    cookies: Option<HashSet<Cookie>>,
    cors: Option<ResponseCors>,
}

impl Respond {
    pub fn builder() -> Builder {
        Builder::default()
    }

}

#[derive(Debug, Default)]
pub struct Builder {
    status: Option<Status>,
    body: Option<Vec<u8>>,
    headers: Option<HashMap<Vec<u8>, Vec<u8>>>,
    cookies: Option<Hashset<Cookie>>,
    cors: Option<ResponseCors>,
}

impl Builder {
    pub fn status(mut self, status: impl Into<Status>) -> Self {
        self.status = Some(status.into());
    }
    pub fn build(self) -> <Respond> {
        Respond {
            
        }
    }

    // checks that status and mime are set before building respond
    fn check_basic_fields() {}

    // checks that cors field is set when we handle a cross origin request
    fn check_cors_field() {}
}

impl Respond {
    pub fn initialize(req: Request, params: &ResourceParams) -> Self {}

    pub fn insert_status_headers(&mut self) {}
}
