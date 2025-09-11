use pheasant_core::{Protocol, Redirection};

pub struct Forward {
    src: String,
    dest: String,
    status: Redirection,
    proto: Protocol,
}

impl Forward {
    fn from_request() {}
}
