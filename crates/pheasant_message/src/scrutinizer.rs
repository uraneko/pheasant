/// validates that the read request's various parts are valid
/// e.g., Pragma: ... header + Http1.1 protocol is an error
///
/// scrutinize a request's contents
pub trait Scrutinizer {
    fn scrutinize(&self) -> Result<(), HttpError>;
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

struct ScrutinizeServerSize {
    buf_size: usize,
    hdr_size: usize,
    hdrs_size: usize,
}

impl<'a> Scrutinizer for ScrutinizeProtoHeaders<'a> {}
