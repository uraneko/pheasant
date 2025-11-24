use super::Socket;
use std::net::TcpListener;

// #[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Builder {
    socket: TcpListener,
    buf_size: usize,
    // service: Option<S>,
    // services: Vec<M>,
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    ServersGottaServe,
}

impl Builder {
    pub fn new(socket: TcpListener) -> Self {
        Self {
            socket,
            buf_size: 4096,
            // service: None,
            // services: Vec::new(),
        }
    }

    pub fn build(self) -> Result<Socket, Error> {
        Ok(Socket {
            socket: self.socket,
            buffer: Vec::with_capacity(self.buf_size),
        })
    }

    pub fn buf_size(mut self, size: usize) -> Self {
        self.buf_size = size;

        self
    }
}
