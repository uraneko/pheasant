use pheasant_prologue::{ErrorStatus, err_stt, message::http11::Lex, server::Request};

pub fn parse(buf: &[u8]) -> Result<Request, ErrorStatus> {
    Lex::new(buf).request().map_err(|_e| err_stt!(BadRequest))
}
