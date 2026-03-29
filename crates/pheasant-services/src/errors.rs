use crate::Content;
use crate::Respond;
use embedded_io::{Read, Write};
use pheasant_http::{ErrorStatus, status};

pub fn http_error(err: ErrorStatus, resp: &mut Respond) {
    resp.status(status!(err.code()));
    // Maybe use seek(0)
    // resp.body_mut().clear();
    let msg = err.text().as_bytes();
    Content::new(msg).dump_headers(resp.headers_mut());
    resp.body_mut().write(msg);
}
