use crate::{AddressFamily, Error, ProtocolNumber, SocketLevel, SocketType};
use core::ffi::c_void;
use pheasant_sys::socket::{
    SockAddr, accept, bind, close, connect, getsockopt, listen, recv, send, shutdown, socket,
    unlink,
};

pub mod io;
pub mod options;

pub use io::{recv::RecvFlags, send::SendFlags};
pub use options::linger;
pub use options::socket::{GetSockOpts, SetSockOpts, SocketOption};

// this trait is a gate keeper
// that makes sure fake sockaddr types: i.e., () (the unit type)
// are allowed for initializing a new Socket instance without a real address
// but have no knowledge of the real socket methods
pub trait TrueSockAddr: SockAddrCasting {}

pub trait SockAddrCasting: Copy {
    const SIZE: u32 = SockAddr::SIZE;
    const ADDRESS_FAMILY: AddressFamily;

    fn cast_ref(&self) -> *const SockAddr {
        self as *const Self as *const SockAddr
    }

    fn cast_mut(&mut self) -> *mut SockAddr {
        self as *mut Self as *mut SockAddr
    }
}

impl SockAddrCasting for () {
    const ADDRESS_FAMILY: AddressFamily = AddressFamily::Inet;
}

pub trait VoidCasting {
    fn cast_ref(&self) -> *const c_void {
        self as *const Self as *const c_void
    }

    fn cast_mut(&mut self) -> *mut c_void {
        self as *mut Self as *mut c_void
    }

    fn size_of(&self) -> u32 {
        core::mem::size_of_val(self) as u32
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Socket<A: SockAddrCasting> {
    sockfd: u32,
    is_bound: bool,
    addr: A,
}

impl Socket<()> {
    pub fn new(
        domain: impl Into<AddressFamily>,
        type_: impl Into<SocketType>,
        proto: impl Into<ProtocolNumber>,
    ) -> Result<Self, Error> {
        let domain = domain.into();
        let type_ = type_.into();
        let proto = proto.into();
        match unsafe { socket(domain.into(), type_.into(), proto.into()) } {
            fd if fd >= 0 => Ok(Self {
                sockfd: fd as u32,
                addr: (),
                is_bound: false,
            }),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn init<A: TrueSockAddr>(self, addr: A) -> Socket<A> {
        Socket {
            sockfd: self.sockfd,
            addr: addr,
            is_bound: false,
        }
    }

    /// connect is for client sockets only
    ///
    /// NOTE: explicitly calling bind before this method is discouraged
    /// let the kernel bind for you
    ///
    /// addr is the server address that you want to connect to
    pub fn connect<A: TrueSockAddr>(&self, addr: &A) -> Result<(), Error> {
        match unsafe { connect(self.fd() as i32, addr.cast_ref(), A::SIZE) } {
            0 => Ok(()),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    /// if you want write
    /// just use this method: send with flags = 0
    ///
    /// NOTE unlike with the server's send
    /// here you dont need to provide a conn_fd socket fd value
    /// as conn_fd here is self.sockfd
    /// assuming that this client socket has successfully established a connection
    /// with the listening socket
    pub fn send(&self, buf: &[u8], flags: impl Into<i32>) -> Result<usize, Error> {
        match unsafe {
            send(
                self.fd() as i32,
                buf.cast_ref(),
                buf.len() as u64,
                flags.into(),
            )
        } {
            n if n >= 0 => Ok(n as usize),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    /// if you want plain read
    /// just use this method with flags = 0
    ///
    /// NOTE unlike with the server's recv
    /// here you dont need to provide a conn_fd socket fd value
    /// as conn_fd here is self.sockfd
    /// assuming that this client socket has successfully established a connection
    /// with the listening socket
    pub fn recv(&self, buf: &mut [u8], flags: impl Into<i32>) -> Result<usize, Error> {
        match unsafe {
            recv(
                self.fd() as i32,
                buf.cast_mut(),
                buf.len() as u64,
                flags.into(),
            )
        } {
            n if n >= 0 => Ok(n as usize),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }
}

impl<A: TrueSockAddr> Socket<A> {
    /// returns true if this socket's address was changed
    /// or false if it was not
    ///
    /// if self.is_bound || self.is_listening == Ok(true)
    /// the operation is abandoned since the address is already bound or in a listening state
    pub fn re_addr(&mut self, addr: A) -> bool {
        if self.is_bound || self.is_listening() == Ok(true) {
            return false;
        }
        self.addr = addr;

        true
    }

    /// returns a new socket instance from the address and fd
    /// the user is responsible for assuring that fd is a proper open socket fd
    pub fn from_params(fd: u32, addr: A, is_bound: bool) -> Self {
        Self {
            sockfd: fd,
            addr: addr,
            is_bound,
        }
    }

    /// returns the socket address family set in/by the socket() call
    pub fn address_family(&self) -> AddressFamily {
        A::ADDRESS_FAMILY
    }

    /// returns true if the socket is in a listening state (listen was called)
    /// else return false
    ///
    /// #### Error
    /// throws if the getsockopt call fails with -1
    pub fn is_listening(&self) -> Result<bool, Error> {
        let mut listening = 0;
        let mut size = listening.size_of();
        match unsafe {
            getsockopt(
                self.fd() as i32,
                SocketLevel::Socket.into(),
                SocketOption::AcceptConn.into(),
                listening.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            0 => Ok(if listening == 0 { false } else { true }),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    /// dont use this with client sockets
    /// only bind if you intend to listen and accept
    pub fn bind(&mut self) -> Result<(), Error> {
        match unsafe { bind(self.fd() as i32, self.addr.cast_ref(), A::SIZE) } {
            0 => self.is_bound = true,
            -1 => return Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }

        Ok(())
    }

    pub fn listen(&self, backlog: u32) -> Result<(), Error> {
        match unsafe { listen(self.fd() as i32, backlog as i32) } {
            0 => Ok(()),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn accept(&self) -> Result<Socket<A>, Error> {
        let mut peer_addr = self.addr.clone();
        let mut size = A::SIZE;
        match unsafe {
            accept(
                self.fd() as i32,
                peer_addr.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            -1 => Err(Error::errno()),
            fd if fd >= 0 => Ok(Socket::from_params(fd as u32, peer_addr, true)),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn shutdown_read(&self) -> Result<(), Error> {
        match unsafe { shutdown(self.fd() as i32, 0) } {
            0 => Ok(()),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn shutdown_write(&self) -> Result<(), Error> {
        match unsafe { shutdown(self.fd() as i32, 1) } {
            0 => Ok(()),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn shutdown_readwrite(&self) -> Result<(), Error> {
        match unsafe { shutdown(self.fd() as i32, 2) } {
            0 => Ok(()),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    /// closes this socket's fd in the system
    /// effectively rendering it useless
    /// thats why this also consumes self
    ///
    /// on success: returns the address that was on self (may or may not have been bound)
    pub fn close(self) -> Result<A, Error> {
        match unsafe { close(self.fd() as i32) } {
            0 => Ok(self.addr),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    /// returns the last socket error if any
    pub fn error(&self) -> Result<Option<Error>, Error> {
        let mut err = 0;
        let mut size = err.size_of();
        match unsafe {
            getsockopt(
                self.fd() as i32,
                SocketLevel::Socket.into(),
                SocketOption::Error.into(),
                err.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            0 if err == 0 => Ok(None),
            0 => Ok(Some(err.into())),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    /// if you want write
    /// just use this method: send with flags = 0
    ///
    /// if you ran a successful accept call
    /// then conn_fd is the sockfd value of the accept return value socket
    pub fn send(&self, conn_fd: u32, buf: &[u8], flags: impl Into<i32>) -> Result<usize, Error> {
        match unsafe {
            send(
                conn_fd as i32,
                buf.cast_ref(),
                buf.len() as u64,
                flags.into(),
            )
        } {
            n if n >= 0 => Ok(n as usize),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    /// if you want plain read
    /// just use this method with flags = 0
    ///
    /// if you ran a successful accept call
    /// then conn_fd is the sockfd value of the accept return value socket
    pub fn recv(
        &self,
        conn_fd: u32,
        buf: &mut [u8],
        flags: impl Into<i32>,
    ) -> Result<usize, Error> {
        match unsafe {
            recv(
                conn_fd as i32,
                buf.cast_mut(),
                buf.len() as u64,
                flags.into(),
            )
        } {
            n if n >= 0 => Ok(n as usize),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }
}

impl<A: SockAddrCasting> Socket<A> {
    /// returns a copy of this socket fd
    pub fn fd(&self) -> u32 {
        self.sockfd
    }

    /// returns the socket_type value used on the socket() call
    pub fn socket_type(&self) -> Result<SocketType, Error> {
        let mut socktype = 0i32;
        let mut size = socktype.size_of();
        match unsafe {
            getsockopt(
                self.fd() as i32,
                SocketLevel::Socket.into(),
                SocketOption::Type.into(),
                socktype.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            0 => socktype.try_into().map_err(|_| Error::errno()),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    /// returns the socket protocol number of the socket fd assigned by the socket() call
    pub fn proto(&self) -> Result<ProtocolNumber, Error> {
        let mut proto = 0i32;
        let mut size = proto.size_of();
        match unsafe {
            getsockopt(
                self.fd() as i32,
                SocketLevel::Socket.into(),
                SocketOption::Protocol.into(),
                proto.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            0 => proto.try_into().map_err(|_| Error::errno()),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }
}

impl Socket<crate::address::SockAddrUn> {
    /// use this instead of close to clean up unix sockets
    pub fn unlink(&self) -> Result<(), Error> {
        match unsafe { unlink(self.addr.path().as_ptr()) } {
            0 => Ok(()),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }
}

// also should be implemented for SockAddrIn6
impl Socket<crate::address::SockAddrIn> {
    pub fn bind_incremental(&mut self) -> Result<u16, Error> {
        while let Err(err) = self.bind() {
            match err {
                Error::AddressInUse => {
                    if self.addr.port == u16::MAX {
                        return Err(err);
                    }

                    self.addr.port += 1;
                }
                err => return Err(err),
            }
        }

        Ok(self.addr.port())
    }
}

impl VoidCasting for u32 {}
impl VoidCasting for i32 {}
impl VoidCasting for bool {}
impl VoidCasting for linger {}
impl VoidCasting for [u8] {}
