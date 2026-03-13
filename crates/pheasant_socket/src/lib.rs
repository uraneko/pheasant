#![no_std]
use pheasant_sys::*;

pub mod address;
pub mod socket;

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

// sources
// '/usr/include/asm-generic/errno.h'
// '/usr/include/asm-generic/errno-base.h'
#[derive(Debug, PartialEq, Eq)]
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
