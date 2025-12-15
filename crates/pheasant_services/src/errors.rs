use crate::MessageBodyInfo;
use pheasant_http::{Respond, status};

pub fn not_found(resp: &mut Respond) {
    resp.status(status!(404));
    let msg = b"server no find thing. server know all good thing at home";
    MessageBodyInfo::new(msg).dump_headers(resp.headers_mut());
    resp.body_mut().extend(msg);
}

pub fn bad_request(resp: &mut Respond) {
    resp.status(status!(400));
    let msg = b"server no like request";
    MessageBodyInfo::new(msg).dump_headers(resp.headers_mut());
    resp.body_mut().extend(msg);
}

pub fn internal_server_error(resp: &mut Respond) {
    resp.status(status!(500));
    let msg = b"server have trouble. server sorry";
    MessageBodyInfo::new(msg).dump_headers(resp.headers_mut());
    resp.body_mut().extend(msg);
}
