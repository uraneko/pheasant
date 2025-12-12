use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub fn read_stream(socket: &TcpListener) -> Result<(TcpStream, SocketAddr), std::io::Error> {
    socket.accept()
}

pub fn req_buf<'a>(reader: &'a mut BufReader<&mut TcpStream>) -> Result<&'a [u8], std::io::Error> {
    reader.fill_buf()
}

pub fn write_stream(buf: &[u8], stream: &mut TcpStream) {
    _ = stream.write(buf);
    _ = stream.flush();
}
