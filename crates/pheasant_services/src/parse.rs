use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn parse(buffer: &mut String, stream: &mut TcpStream) {
    let mut reader = BufReader::new(stream);
    buffer.clear();
    loop {
        _ = reader.read_line(buffer);
        if buffer.ends_with("\r\n\r\n") {
            break;
        }
    }
}
