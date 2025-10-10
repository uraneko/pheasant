use super::{MessageFragment, Request};
use pheasant_core::{ErrorStatus, Informational};

pub struct Negotiate {
    status: Informational,
}

impl Negotiate {
    pub fn initialize(req: Request) -> Self {
        todo!()
    }
}

impl<E: ErrorStatus> From<Negotiate> for MessageFragment<'_, E> {
    fn from(neg: Negotiate) -> Self {
        MessageFragment::Negotiate(neg)
    }
}
