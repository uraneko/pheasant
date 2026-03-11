use pheasant_sys::*;

pub struct Builder {
    address_family: AddressFamily,
    socket_type: SocketType,
    protocol_number: ProtocolNumber,
}

pub trait Socket {
    /// returns the protocol stack of the socket
    fn stack(&self) -> &[SocketLevel];
}

impl Builder {
    fn build<S: Socket>(self, socket_address: impl Into<SockAddr>) -> S {
        todo!()
    }
}

impl Socket for TcpSocket {
    fn stack(&self) -> &[SocketLevel] {
        &[]
    }
}

pub struct TcpSocket {
    fd: u32,
    addr: u32,
    port: u16,
}

impl TcpSocket {
    pub fn new() -> Self {
        todo!()
    }
}
