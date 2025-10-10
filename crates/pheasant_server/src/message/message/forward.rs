use super::{MessageFragment, Request};
use hashbrown::HashMap;
use pheasant_core::{ErrorStatus, Protocol, Redirection};
use pheasant_uri::Route;

pub struct Forward {
    src: String,
    dest: String,
    status: Redirection,
    proto: Protocol,
    headers: HashMap<String, String>,
}

impl Forward {
    pub fn initialize(req: Request, dest: Route) -> Self {
        todo!()
    }
}

impl<E: ErrorStatus> From<Forward> for MessageFragment<E> {
    fn from(frd: Forward) -> Self {
        MessageFragment::Forward(frd)
    }
}
