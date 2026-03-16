use super::{Error, SockAddrIn, Socket, TcpSocket};
use sqlx::{ConnectOptions, sqlite};

// #[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Builder {
    socket: TcpSocket<SockAddrIn>,
    buf_size: usize,
    incremental_bind: bool,
    path: &'static str,
}

impl Builder {
    pub fn new(socket: TcpSocket<SockAddrIn>) -> Self {
        Self {
            socket,
            buf_size: 4096,
            incremental_bind: false,
            path: ":memory:",
        }
    }

    /// builds the server socket
    /// this also binds the socket and sets it in a listening state
    pub async fn build(mut self) -> Result<Socket, Error> {
        if self.incremental_bind {
            self.socket.bind_incremental()?;
        } else {
            self.socket.bind()?;
        }
        self.socket.listen(128)?;

        Ok(Socket {
            socket: self.socket,
            buffer: vec![0; self.buf_size],
            cursor: 0,
            conn: sqlite::SqliteConnectOptions::new()
                .filename(self.path)
                .create_if_missing(true)
                .connect()
                .await
                .map_err(|_| Error::SqliteConnFailed)?,
        })
    }

    /// turning this on means that
    pub fn incremental_bind(mut self, increm: bool) -> Self {
        self.incremental_bind = increm;

        self
    }

    /// specifies the path to the database file
    pub fn database(mut self, path: &'static str) -> Self {
        self.path = path;

        self
    }

    /// specifies the socket's buffer size
    pub fn buf_size(mut self, size: usize) -> Self {
        self.buf_size = size;

        self
    }
}
