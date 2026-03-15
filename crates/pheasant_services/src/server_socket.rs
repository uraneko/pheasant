use crate::TcpSocket;
use pheasant_socket::{
    AddressFamily, Error as SocketError, ProtocolNumber, SocketType, address::SockAddrIn,
    socket::SetSockOpts,
};
use sqlx::sqlite;

pub mod builder;

pub struct Socket {
    pub socket: TcpSocket<SockAddrIn>,
    pub buffer: Vec<u8>,
    pub conn: sqlite::SqliteConnection,
}

impl Socket {
    /// # Error
    /// returns an Err if the Tcp Listener generation fails
    pub fn builder(host: &str) -> Result<builder::Builder, Error> {
        let socket = TcpSocket::new(
            AddressFamily::Inet,
            SocketType::Stream,
            ProtocolNumber::Default,
        )?;
        SetSockOpts::new(socket.fd()).reuse_address(true)?;
        let addr = host.parse().map_err(|_| Error::BadAddrOrPort)?;

        Ok(builder::Builder::new(socket.init::<SockAddrIn>(addr)))
    }

    // / prints out the socket url on the stdout
    // fn init_message(&self) -> String {
    //     "\x1b[1;38;2;211;163;104mSocket listening on http://{}:{}\x1b[0m\r\n";
    // }

    /// returns a copy of self.socket
    /// which implements send and recv methods
    /// to exchange messages with another socket
    pub fn inner(&self) -> TcpSocket<SockAddrIn> {
        self.socket
    }

    pub fn buf_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }

    pub fn buf_ref(&self) -> &[u8] {
        &self.buffer
    }
}

impl crate::Server for Socket {}

pub enum Error {
    Socket(SocketError),
    BadAddrOrPort,
    SqliteConnFailed,
}

impl From<SocketError> for Error {
    fn from(err: SocketError) -> Self {
        Self::Socket(err)
    }
}
