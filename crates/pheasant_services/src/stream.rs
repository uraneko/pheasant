use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub fn read_stream(socket: &TcpListener) -> Result<(TcpStream, SocketAddr), std::io::Error> {
    socket.accept()
}

pub fn write_stream(buf: &str, stream: &mut TcpStream) {
    _ = stream.write(buf.as_bytes());
    _ = stream.flush();
}
