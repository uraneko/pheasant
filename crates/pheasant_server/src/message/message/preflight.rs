use super::{MessageFragment, Request};
use hashbrown::HashMap;
use pheasant_core::{ErrorStatus, Protocol, Successful};
use pheasant_headers::ResponseCors;

// TODO An origin server that does not support persistent connections MUST send the Connection: close in every response that does not have a 1xx status code.
// TODO An origin server MUST generate an Allow header in a 405 (Method Not Allowed) response.
// TODO An origin server generating a 401 (Unauthorized) response MUST send a WWW-Authenticate header field containing at least one challenge.

pub struct Preflight {
    proto: Protocol,
    status: Successful,
    headers: HashMap<String, String>,
    // cors: ResponseCors<'a>,
}

impl Preflight {
    pub fn initialize<'a>(req: Request, cors: ResponseCors<'a>) -> Self {
        todo!()
    }
}

impl<E: ErrorStatus> From<Preflight> for MessageFragment<E> {
    fn from(prf: Preflight) -> Self {
        Self::Preflight(prf)
    }
}
