use pheasant_core::{Protocol, Successful};

pub struct Preflight {
    proto: Protocol,
    status: Successful,
    cors: ResponseCors,
}
