use pheasant_core::{ErrorStatus, Method, Protocol};

struct ErrorMessage<E> {
    resource: String,
    method: Method,
    proto: Protocol,
    // status can be server error or client error
    status: E,
}

impl<E> ErrorMessage<E> {
    pub fn from_err(err: E) -> Self {}
}
