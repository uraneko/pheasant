use super::{Error, Socket};
use std::net::TcpListener;

// #[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Builder {
    socket: Result<TcpListener, Error>,
    buf_size: usize,
}

impl Builder {
    pub fn new(socket: Result<TcpListener, Error>) -> Self {
        Self {
            socket,
            buf_size: 4096,
        }
    }

    pub fn build(self) -> Result<Socket, Error> {
        Ok(Socket {
            socket: self.socket?,
            buffer: Vec::with_capacity(self.buf_size),
        })
    }

    pub fn buf_size(mut self, size: usize) -> Self {
        self.buf_size = size;

        self
    }
}
