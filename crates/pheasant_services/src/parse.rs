use pheasant_prologue::{ErrorStatus, err_stt, message::http11::Lex, server::Request};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn parse(buf: &[u8]) -> Result<Request, ErrorStatus> {
    Lex::new(buf).request().map_err(|_e| err_stt!(BadRequest))
}

pub fn parse_naive(buffer: &mut String, stream: &mut TcpStream) {
    let mut reader = BufReader::new(stream);
    buffer.clear();
    loop {
        _ = reader.read_line(buffer);
        if buffer.ends_with("\r\n\r\n") {
            break;
        }
    }
}
