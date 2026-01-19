use super::{Error, Socket};
use sqlx::{Connection, sqlite};
use std::net::TcpListener;

// #[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Builder {
    socket: Result<TcpListener, Error>,
    buf_size: usize,
    path: &'static str,
}

impl Builder {
    pub fn new(socket: Result<TcpListener, Error>) -> Self {
        Self {
            socket,
            buf_size: 4096,
            path: ":memory:",
        }
    }

    pub async fn build(self) -> Result<Socket, Error> {
        Ok(Socket {
            socket: self.socket?,
            buffer: Vec::with_capacity(self.buf_size),
            conn: sqlite::SqliteConnection::connect(self.path).await.unwrap(),
        })
    }

    pub fn database(mut self, path: &'static str) -> Self {
        self.path = path;

        self
    }

    pub fn buf_size(mut self, size: usize) -> Self {
        self.buf_size = size;

        self
    }
}
