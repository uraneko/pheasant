use pheasant_core::{Protocol, Successful};
use pheasant_headers::ResponseCors;

pub struct Preflight<'a> {
    proto: Protocol,
    status: Successful,
    cors: ResponseCors<'a>,
}
