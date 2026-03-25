use crate::Content;
use embedded_io::{Read, Write};
use pheasant_prologue::{ErrorStatus, server::Respond, status};

pub fn http_error<HRW: Read + Write, BRW: Read + Write>(
    err: ErrorStatus,
    resp: &mut Respond<HRW, BRW>,
) {
    resp.status(status!(err.code()));
    // Maybe use seek(0)
    // resp.body_mut().clear();
    let msg = err.text().as_bytes();
    Content::new(msg).dump_headers(resp.headers_mut());
    resp.body_mut().write(msg);
}
