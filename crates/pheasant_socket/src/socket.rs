use core::ffi::c_void;
use pheasant_sys::*;

pub mod options;

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
    address: A,
}

impl<A: SockAddrCasting> Socket<A> {
    pub fn fd(&self) -> u32 {
        self.sockfd
    }

    pub fn new(fd: u32, addr: A) -> Self {
        Self {
            sockfd: fd,
            address: addr,
        }
    }

    /// dont use this with client sockets
    /// only bind if you intend to to listen and accept
    pub fn bind(&self) -> Result<(), Error> {
        match unsafe { bind(self.fd() as i32, self.address.cast_ref(), A::SIZE) } {
            0 => Ok(()),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    /// connect is for client sockets only
    ///
    /// NOTE: explicitly calling bind before this method is discouraged
    pub fn connect(&self, addr: &A) -> Result<(), Error> {
        match unsafe { connect(self.fd() as i32, addr.cast_ref(), A::SIZE) } {
            0 => Ok(()),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn listen(&self, backlog: u32) -> Result<(), Error> {
        match unsafe { listen(self.fd() as i32, backlog as i32) } {
            0 => Ok(()),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn accept(&self) -> Result<Socket<A>, Error> {
        let mut peer_addr = self.address.clone();
        let mut size = A::SIZE;
        match unsafe {
            accept(
                self.fd() as i32,
                peer_addr.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            -1 => {
                extern crate std;

                Err(Error::errno())
            }
            fd if fd >= 0 => Ok(Socket::new(fd as u32, peer_addr)),
            err => unreachable!("unexpected error code {}", err),
        }
    }
}

impl<A: SockAddrCasting> Socket<A> {
    /// returns the socket_type value used on the socket() call
    pub fn socket_type(&self) -> Result<SocketType, Error> {
        let mut socktype = 0i32;
        let mut size = socktype.size_of();
        match unsafe {
            getsockopt(
                self.fd() as i32,
                SocketLevel::Socket.into_int(),
                SocketOption::Type.into_int(),
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
                SocketLevel::Socket.into_int(),
                SocketOption::Protocol.into_int(),
                proto.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            0 => proto.try_into().map_err(|_| Error::errno()),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
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
                SocketLevel::Socket.into_int(),
                SocketOption::AcceptConn.into_int(),
                listening.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            0 => Ok(if listening == 0 { false } else { true }),
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
                SocketLevel::Socket.into_int(),
                SocketOption::Error.into_int(),
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
}

impl VoidCasting for u32 {}
impl VoidCasting for i32 {}
impl VoidCasting for bool {}
impl VoidCasting for linger {}

// sources
// '/usr/include/asm-generic/errno.h'
// '/usr/include/asm-generic/errno-base.h'
#[derive(Debug)]
#[repr(i32)]
pub enum Error {
    // EPERM
    OperationNotPermitted = 1,
    // ENOENT
    FileOrDirDoesntExist = 2,
    // EINTR
    SysCalInterrupted = 4,
    // EBADF
    BadFileNumber = 9,
    // EWOULDBLOCK / EAGAIN
    // TryAgain = 11,
    OperationWouldBlock = 11,
    // ENOMEM
    OutOfMemory = 12,
    // EACCES
    PermissionDenied = 13,
    // EFAULT
    BadAddress = 14,
    // ENOTDIR
    NotADir = 20,
    // EINVAL
    InvalidArgument = 22,
    // ENFILE
    FileTableOverflow = 23,
    // EMFILE
    TooManyOpenFiles = 24,
    // EROFS
    ReadOnlyFileSystem = 30,
    // ENAMETOOLONG
    FileNameTooLong = 36,
    // ELOOP
    TooManySymLinks = 40,
    // EPROTO
    ProtocolError = 71,
    // ENOTSOCK
    SocketOperationOnNonSocket = 88,
    // EPROTONOSUPPORT
    UnsupportedProtocol = 93,
    // EOPNOTSUPP
    OperationNotSupported = 95,
    // EAFNOSUPPORT
    AddressFamilyNotSupportedByProto = 97,
    // EADDRINUSE
    AddressInUse = 98,
    // EADDRNOTAVAIL
    CantAssignRequestedAddr = 99,
    // ENETUNREACH
    NetworkUnreachable = 101,
    // ECONNABORTED
    ConnectionAborted = 103,
    // ENOBUFS
    NoBufferSpaceAvailable = 105,
    // EISCONN
    TransportEndpointAlreadyConnected = 106,
    // ENOTCONN
    TransportEndpointNotConnected = 107,
    // ETIMEDOUT
    ConnectionTimedOut = 110,
    // ECONNREFUSED
    ConnectionRefused = 111,
    // EALREADY
    OperationAlreadyInProgress = 114,
    // EINPROGRESS
    OperationNowInProgress = 115,
    Other(i32),
}

impl From<i32> for Error {
    fn from(err: i32) -> Self {
        match err {
            1 => Self::OperationNotPermitted,
            2 => Self::FileOrDirDoesntExist,
            4 => Self::SysCalInterrupted,
            9 => Self::BadFileNumber,
            11 => Self::OperationWouldBlock,
            12 => Self::OutOfMemory,
            13 => Self::PermissionDenied,
            14 => Self::BadAddress,
            20 => Self::NotADir,
            22 => Self::InvalidArgument,
            23 => Self::FileTableOverflow,
            24 => Self::TooManyOpenFiles,
            30 => Self::ReadOnlyFileSystem,
            36 => Self::FileNameTooLong,
            40 => Self::TooManySymLinks,
            71 => Self::ProtocolError,
            88 => Self::SocketOperationOnNonSocket,
            93 => Self::UnsupportedProtocol,
            95 => Self::OperationNotSupported,
            97 => Self::AddressFamilyNotSupportedByProto,
            98 => Self::AddressInUse,
            99 => Self::CantAssignRequestedAddr,
            101 => Self::NetworkUnreachable,
            103 => Self::ConnectionAborted,
            105 => Self::NoBufferSpaceAvailable,
            106 => Self::TransportEndpointAlreadyConnected,
            107 => Self::TransportEndpointNotConnected,
            110 => Self::ConnectionTimedOut,
            111 => Self::ConnectionRefused,
            114 => Self::OperationAlreadyInProgress,
            115 => Self::OperationNowInProgress,
            err => Self::Other(err),
        }
    }
}

impl From<Option<i32>> for Error {
    fn from(opt: Option<i32>) -> Self {
        let Some(err) = opt else {
            return Self::Other(-1);
        };

        err.into()
    }
}

extern crate std;
impl Error {
    fn errno() -> Self {
        let err = std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap_or_else(|| 0);
        err.into()
    }
}
