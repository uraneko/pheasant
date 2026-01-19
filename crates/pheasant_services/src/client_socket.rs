use crate::Service;
use pheasant_http::server::Request;
use std::io::Result as IoRes;
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};

pub struct ClientSocket {
    addr: Ipv4Addr,
    port: u16,
}

pub enum Error {
    ConnectFailed,
}

impl ClientSocket {
    pub fn new(addr: impl Into<Ipv4Addr>, port: u16) -> Self {
        let addr = addr.into();
        Self { addr, port }
    }

    pub fn addr(&mut self, addr: impl Into<Ipv4Addr>) {
        self.addr = addr.into();
    }

    pub fn port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn socket(&self) -> Result<TcpStream, Error> {
        TcpStream::connect((self.addr, self.port)).map_err(|_| Error::ConnectFailed)
    }
}
