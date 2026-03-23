use crate::Content;
use pheasant_prologue::{ErrorStatus, server::Respond, status};

pub fn http_error(err: ErrorStatus, resp: &mut Respond) {
    resp.status(status!(err.code()));
    resp.body_mut().clear();
    let msg = err.text().as_bytes();
    Content::new(msg).dump_headers(resp.headers_mut());
    resp.body_mut().extend(msg);
}
