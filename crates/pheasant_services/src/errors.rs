use pheasant_http::{Respond, status};

pub fn not_found(resp: &mut Respond) {
    resp.status(status!(404));
    resp.headers_mut()
        .extend(b"content-type: text/plain\ncontent-length: 14\n");
    resp.body_mut().extend(b"not found haha");
}

pub fn bad_request(resp: &mut Respond) {
    resp.status(status!(400));
    resp.headers_mut()
        .extend(b"Content-Type: text/plain\nContent-Length: 14\n");
    resp.body_mut().extend(b"bad request haha");
}
