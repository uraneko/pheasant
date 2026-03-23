use crate::TcpSocket;
use pheasant_socket::{
    AddressFamily, Error as SocketError, ProtocolNumber, SocketType, address::SockAddrIn,
};

pub struct Socket {
    socket: TcpSocket<()>,
    buffer: Vec<u8>,
}

impl Socket {
    pub fn new(buf_size: usize) -> Result<Self, Error> {
        Ok(Self {
            socket: TcpSocket::new(
                AddressFamily::Inet,
                SocketType::Stream,
                ProtocolNumber::Default,
            )?,
            buffer: Vec::with_capacity(buf_size),
        })
    }

    pub fn connect(&self, server: &str) -> Result<(), Error> {
        let addr = server
            .parse::<SockAddrIn>()
            .map_err(|_| Error::BadAddrStr)?;
        self.socket.connect(&addr)?;

        Ok(())
    }

    /// returns a copy of self.socket
    /// which implements send and recv methods
    /// to exchange messages with another socket
    pub fn inner(&self) -> TcpSocket<()> {
        self.socket
    }

    pub fn buf_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }

    pub fn buf_ref(&self) -> &[u8] {
        &self.buffer
    }
}

#[derive(Debug)]
pub enum Error {
    Socket(SocketError),
    BadAddrStr,
    ConnectFailed,
}

impl From<SocketError> for Error {
    fn from(err: SocketError) -> Self {
        Self::Socket(err)
    }
}
