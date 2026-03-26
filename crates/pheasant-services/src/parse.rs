use pheasant_prologue::{
    ErrorStatus, client::Respond, err_stt, message::http11::Lex, server::Request,
};

pub fn request(buf: &[u8]) -> Result<Request, ErrorStatus> {
    Lex::new(buf).request().map_err(|_e| err_stt!(BadRequest))
}

pub fn respond(buf: &[u8]) -> Result<Respond, ErrorStatus> {
    Lex::new(buf).respond().map_err(|_| err_stt!(BadRequest))
}
