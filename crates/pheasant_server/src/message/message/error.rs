use pheasant_core::{
    ClientError, ErrorStatus, ErrorStatus, Method, Protocol, ServerError, c_err, s_err,
};

pub struct ErrorMessage<E> {
    proto: Protocol,
    // status can be server error or client error
    status: E,
}

impl<E: ErrorStatus> ErrorMessage<E> {
    pub fn from_err(proto: Protocol, err: E) -> Self {
        Self { status: err, proto }
    }
}
