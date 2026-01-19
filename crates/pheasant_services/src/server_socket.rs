use crate::Service;
use pheasant_http::server::Request;
use sqlx::sqlite;
use std::io::Result as IoRes;
use std::net::{Ipv4Addr, SocketAddr, TcpListener};

pub mod builder;

/// tries to bind the socket to the passed addr and port
/// keeps incrementing port number until it finds a free port
///
/// ### Error
/// - returns an std::io::Error when port reaches u16::MAX and no free port is found
pub fn bind_socket(addr: impl Into<Ipv4Addr>, mut port: u16) -> Result<TcpListener, Error> {
    let addr = addr.into();
    let socket = loop {
        match TcpListener::bind((addr, port)) {
            Ok(listener) => break listener,
            err if port == u16::MAX => return err.map_err(|_| Error::None),
            _err => port += 1,
        }
    };

    Ok(socket)
}

pub struct Socket {
    pub socket: TcpListener,
    pub buffer: Vec<u8>,
    pub conn: sqlite::SqliteConnection,
}

impl Socket {
    /// # Error
    /// returns an Err if the Tcp Listener generation fails
    pub fn builder(addr: impl Into<Ipv4Addr>, port: u16) -> builder::Builder {
        let socket = bind_socket(addr, port);

        builder::Builder::new(socket)
    }
}

impl crate::Server for Socket {
    // returns a result of the socket's ip addr
    fn addr(&self) -> IoRes<SocketAddr> {
        self.socket.local_addr()
    }

    fn port(&self) -> u16 {
        match self.socket.local_addr() {
            Ok(addr) => addr.port(),
            Err(_) => 80,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    None,
    ServersGottaServe,
}
