use crate::TcpSocket;
use pheasant_socket::{
    AddressFamily, Error as SocketError, ProtocolNumber, SocketType, address::SockAddrIn,
};

type Request = pheasant_http::Request<Vec<u8>>;

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

    pub fn write(&mut self, req: Request) -> Result<usize, Error> {
        self.buffer.clear();
        self.buffer.extend(req.stream_bytes());

        let n = self.socket.send(&mut self.buffer, 0)?;

        Ok(n)
    }

    pub fn read(&mut self) -> Result<usize, Error> {
        self.buffer = vec![0u8; 4096];
        let n = self.socket.recv(&mut self.buffer, 0)?;

        Ok(n)
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
