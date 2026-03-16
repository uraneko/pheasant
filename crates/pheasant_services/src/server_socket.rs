use crate::TcpSocket;
use pheasant_prologue::{Method, server::Respond};
use pheasant_socket::{
    AddressFamily, Error as SocketError, ProtocolNumber, SocketType, address::SockAddrIn,
    socket::SetSockOpts,
};
use sqlx::sqlite;

pub mod builder;

#[derive(Debug)]
pub struct Socket {
    pub socket: TcpSocket<SockAddrIn>,
    pub buffer: Vec<u8>,
    cursor: usize,
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
    pub fn init_message(&self) -> String {
        let [o0, o1, o2, o3] = self.socket.addr();
        format!(
            "\x1b[1;38;2;211;163;104mSocket listening on http://{}.{}.{}.{}:{}\x1b[0m\r\n",
            o0,
            o1,
            o2,
            o3,
            self.socket.port()
        )
    }

    /// returns a copy of self.socket
    /// which implements send and recv methods
    /// to exchange messages with another socket
    pub fn inner(&self) -> TcpSocket<SockAddrIn> {
        self.socket
    }

    pub fn read(&mut self, fd: u32) -> Result<(), Error> {
        self.clr_buf();
        let socket = self.socket;
        self.cursor = socket.recv(fd, self.buf_mut(), 0)?;

        Ok(())
    }

    pub fn dump(&mut self, resp: &Respond, method: Method) {
        self.clr_buf();
        self.cursor = resp.dump_bytes(self.buf_mut(), method);
    }

    pub fn write(&mut self, fd: u32) -> Result<(), Error> {
        let socket = self.socket;
        assert!(self.cursor == socket.send(fd, self.buf_ref(), 0)?);
        self.clr_buf();

        Ok(())
    }

    pub fn buf_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }

    pub fn buf_ref(&self) -> &[u8] {
        &self.buffer[..self.cursor]
    }

    pub fn clr_buf(&mut self) {
        self.buffer.fill(0);
        self.cursor = 0;
    }
}

impl crate::Server for Socket {}

#[derive(Debug)]
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
