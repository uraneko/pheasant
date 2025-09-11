/// validates that the read request's various parts are valid
/// e.g., Pragma: ... header + Http1.1 protocol is an error
///
/// scrutinize a request's contents
pub trait Scrutinizer {
    fn scrutinize(&self) -> Result<(), ErrorStatus>;
}
