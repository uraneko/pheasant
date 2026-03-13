use pheasant_sys::*;

// Socket is the name of the unsafe ffi function socket
pub enum Error {
    AccessDenied,
    UnsupportedAf,
    UnknownProto,
    FileTableOverflow,
    SysOpenFileLimitReached,
    InsufficientMemory,
    UnsupportedProto,
}

pub struct Socket {
    domain: ProtocolFamily,
    type_: SocketType,
    proto: ProtocolNumber,
}

impl Socket {
    pub fn new() -> Self {
        Self {
            domain: None,
            type_: None,
            proto: None,
        }
    }

    pub fn acquire(self) -> Result<u32, Error> {
        let domain = self.domain.ok_or_else(|| Error::UnsufficientParameters)?;
        let type_ = self.type_.ok_or_else(|| domain.preferred_socket_type())?;
        let proto = self.proto.ok_or_else(|| type_.preferred_protocol())?;
        let res = unsafe { socket(domain.as_int(), type_.as_int(), proto.as_int()) };
        if res == -1 {
            extern crate std;

            let error = std::io::Error::last_os_error();
            todo!()
        }

        Ok(res)
    }
}
