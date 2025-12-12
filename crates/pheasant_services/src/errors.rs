pub fn not_found(buf: &mut Vec<u8>) {
    buf.clear();
    buf.extend(
        b"HTTP/1.1 404 Not Found\nContent-Type: text/plain\nContent-Length: 14\n\nnot found haha",
    );
}

pub fn bad_request(buf: &mut Vec<u8>) {
    buf.clear();
    buf.extend(
        b"HTTP/1.1 400 Bad Request\nContent-Type: text/plain\nContent-Length: 14\n\nbad request haha"
    );
}
