pub fn not_found(buf: &mut String) {
    *buf = format!(
        "HTTP/1.1 404 Not Found\nContent-Type: text/plain\nContent-Length: 14\n\nnot found haha"
    );
}

pub fn bad_request(buf: &mut String) {
    *buf = format!("");
}
