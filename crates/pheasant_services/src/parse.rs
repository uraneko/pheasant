use pheasant_http::{
    ErrorStatus, err_stt,
    request::{Request, http11::Lex},
};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn parse(buf: &[u8]) -> Result<Request, ErrorStatus> {
    let mut lex = Lex::new(buf);

    lex.request().map_err(|_e| err_stt!(BadRequest))
}

pub fn parse2(buffer: &mut String, stream: &mut TcpStream) {
    let mut reader = BufReader::new(stream);
    buffer.clear();
    loop {
        _ = reader.read_line(buffer);
        if buffer.ends_with("\r\n\r\n") {
            break;
        }
    }
}
