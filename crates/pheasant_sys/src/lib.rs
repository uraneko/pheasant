#![no_std]
#![allow(non_camel_case_types)]
//! socket domain and type definitions can be found at
//! /usr/include/bits/socket.h
//! #include <sys/socket.h>
//! #include <netinet/in.h>
//! #include <arpa/inet.h>

use core::ffi::{CStr, c_int, c_uint, c_void};
pub mod sockaddr;
pub use sockaddr::{InAddr, SockAddr, SockAddrIn, in_addr_t};

unsafe extern "C" {
    pub fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;

    pub fn bind(sockfd: c_int, socket_address: *const SockAddr, socketlen_t: c_uint) -> c_int;

    // socket len is size of socket's address structure type
    pub fn connect(sockfd: c_int, socket_address: *const SockAddr, socketlen_t: c_uint) -> c_int;

    pub fn listen(sockfd: c_int, backlog: c_int) -> c_int;

    pub fn accept(sockfd: c_int, sockaddr: *mut SockAddr, socklen: *mut c_uint) -> c_int;

    pub fn setsockopt(
        sockfd: c_int,
        level: c_int,
        name: c_int,
        value: *const c_void,
        len: socklen_t,
    ) -> c_int;

    pub fn getsockopt(
        sockfd: c_int,
        level: c_int,
        name: c_int,
        value: *mut c_void,
        len: *mut socklen_t,
    ) -> c_int;

    pub fn read(sockfd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;

    pub fn write(sockfd: c_int, buf: *const c_void, count: size_t) -> ssize_t;

    pub fn recv(sockfd: c_int, buf: *mut c_void, count: size_t, flags: c_int) -> ssize_t;

    pub fn send(sockfd: c_int, buf: *const c_void, count: size_t, flags: c_int) -> ssize_t;

    pub fn inet_addr(addr: *const i8) -> in_addr_t;
}

#[repr(C)]
pub enum SendFlag {
    MsgConfirm = 2048,
    MsgDontroute = 4,
    MsgDontwait = 64,
    MsgEor = 128,
    MsgMore = 32768,
    MsgNosignal = 16384,
    MsgOob = 1,
}

impl SendFlag {
    pub fn to_int(&self) -> c_int {
        use SendFlag::*;

        match self {
            MsgConfirm => 2048,
            MsgDontroute => 4,
            MsgDontwait => 64,
            MsgEor => 128,
            MsgMore => 32768,
            MsgNosignal => 16384,
            MsgOob => 1,
        }
    }

    pub fn union(flags: &[Self]) -> c_int {
        flags.into_iter().fold(0, |acc, f| acc | f.to_int())
    }
}

#[repr(C)]
pub enum RecvFlag {
    MsgDontwait = 64,
    MsgErrqueue = 8192,
    MsgOob = 1,
    MsgPeek = 2,
    MsgTrunc = 32,
    MsgWaitall = 256,
}

impl RecvFlag {
    pub fn to_int(&self) -> c_int {
        use RecvFlag::*;

        match self {
            MsgDontwait => 64,
            MsgErrqueue => 8192,
            MsgOob => 1,
            MsgPeek => 2,
            MsgTrunc => 32,
            MsgWaitall => 256,
        }
    }

    pub fn union(flags: &[Self]) -> c_int {
        flags.into_iter().fold(0, |acc, f| acc | f.to_int())
    }
}
type size_t = u64;
type ssize_t = u64;

#[repr(C)]
pub enum SocketLevel {
    // this sets/gets the option for the socket level itself not a deeper protocol
    // whatever that means
    Socket = 1,
    Ip = 0,
    Ipv6 = 41,
    Icmpv6 = 58,
    Raw = 255,
}

impl SocketLevel {
    pub fn into_int(&self) -> c_int {
        use SocketLevel::*;

        match self {
            Ip => 0,
            Socket => 1,
            Ipv6 => 41,
            Icmpv6 => 58,
            Raw => 255,
        }
    }
}

#[repr(C)]
// these are all SOL_SOCKET level options
// different levels have different options
pub enum SocketOption {
    // SO_DEBUG
    Debug = 1,
    // SO_REUSEADDR
    ReuseAddr = 2,
    // SO_TYPE
    Type = 3,
    // SO_ERROR
    Error = 4,
    // SO_DONTROUTE
    DontRoute = 5,
    Broadcast = 6,
    SndBuf = 7,
    RcvBuf = 8,
    KeepAlive = 9,
    OOBInline = 10,
    Linger = 13,
    ReusePort = 15,
    RcvLowAT = 18,
    SndLowAT = 19,
    AcceptConn = 30,
    Protocol = 38,
    // SO_RCVTIMEO
    RcvTimeOut = 66,
    // SO_SNDTIMEO
    SndTimeOut = 67,
}

impl SocketOption {
    pub fn into_int(&self) -> c_int {
        use SocketOption::*;
        match self {
            Debug => 1,
            ReuseAddr => 2,
            Type => 3,
            Error => 4,
            DontRoute => 5,
            Broadcast => 6,
            SndBuf => 7,
            RcvBuf => 8,
            KeepAlive => 9,
            OOBInline => 10,
            Linger => 13,
            ReusePort => 15,
            RcvLowAT => 18,
            SndLowAT => 19,
            AcceptConn => 30,
            Protocol => 38,
            RcvTimeOut => 66,
            SndTimeOut => 67,
        }
    }
}

type socklen_t = c_uint;

// TODO
unsafe extern "C" {
    pub fn perror(msg: *const u8);

    pub fn strerror();

    pub fn fcntl(fd: c_int, cmd: c_int) -> c_int;

    pub fn ioctl(d: c_int, request: c_int, ...) -> c_int;
}

pub struct AcquireSockFd {
    domain: ProtocolFamily,
    type_: SocketType,
    proto: ProtocolNumber,
}

impl AcquireSockFd {
    pub fn new<T>(domain: ProtocolFamily, type_: SocketType, proto: T) -> Self
    where
        T: TryInto<ProtocolNumber, Error: core::fmt::Debug>,
    {
        Self {
            domain,
            type_,
            proto: proto.try_into().unwrap(),
        }
    }

    pub fn acquire(self) -> c_int {
        unsafe {
            socket(
                self.domain.into_int(),
                self.type_.into_int(),
                self.proto.into_int(),
            )
        }
    }
}

// values gotten from /usr/include/bits/socket.h
// under /* Protocol families.  */ definitions
// AF_* definitions are just aliases for the PF_* definitions there
// DOCS using Af* syntax instead of Pf since 'https://linux.die.net/man/2/socket' claims:
// However, already the BSD man page promises:
// "The protocol family generally is the same as the address family",
// and subsequent standards use AF_* everywhere.
#[derive(Debug, Clone, Copy)]
pub enum ProtocolFamily {
    // for local communications
    AfUnix, // aka AF_LOCAL
    // for ipv4
    AfInet,
    // for ipv6
    AfInet6,
    AfIpx,
    AfNetlinK,
    AfX25,
    AfAx25,
    AfAtmpvc,
    AfAppletalk,
    AfPacket,
}

pub type AddressFamily = ProtocolFamily;

impl ProtocolFamily {
    pub fn into_int(&self) -> c_int {
        use ProtocolFamily::*;

        match self {
            AfUnix => 1,
            AfInet => 2,
            AfInet6 => 10,
            AfIpx => 4,
            AfNetlinK => 16,
            AfX25 => 9,
            AfAx25 => 3,
            AfAtmpvc => 8,
            AfAppletalk => 5,
            AfPacket => 17,
        }
    }
}

// values retrieved from bits/socket_type.h
// under section /* Types of sockets.  */
#[derive(Debug, Clone, Copy)]
pub enum SocketType {
    // use this to open a tcp socket
    SockStream,
    // use this to open a udp socket
    SockDgram,
    SockSeqPacket,
    // use this for direct access to the underlying ip protocol
    SockRaw,
    SockRdm,
}
// sock type conv fail
#[derive(Debug)]
pub enum ConversionError {
    BadInt(i32),
}

impl TryFrom<i32> for SocketType {
    type Error = ConversionError;
    fn try_from(int: i32) -> Result<Self, Self::Error> {
        Ok(match int {
            1 => Self::SockStream,
            2 => Self::SockDgram,
            3 => Self::SockRaw,
            4 => Self::SockRdm,
            5 => Self::SockSeqPacket,
            int => return Err(ConversionError::BadInt(int)),
        })
    }
}

impl SocketType {
    pub fn into_int(&self) -> c_int {
        use SocketType::*;

        match self {
            SockStream => 1,
            SockDgram => 2,
            SockRaw => 3,
            SockRdm => 4,
            SockSeqPacket => 5,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ProtocolNumber {
    #[default]
    SocketTypeDefault,
    Icmp,
    Ipv4,
    Ipv6,
    Tcp,
    Udp,
    Else(c_int),
}

impl ProtocolNumber {
    pub fn into_int(&self) -> c_int {
        match self {
            Self::SocketTypeDefault => 0,
            Self::Ipv4 => 4,
            Self::Ipv6 => 41,
            Self::Tcp => 6,
            Self::Udp => 17,
            Self::Icmp => 1,
            Self::Else(num) => *num,
        }
    }
}

impl TryFrom<i32> for ProtocolNumber {
    type Error = ConversionError;
    fn try_from(int: i32) -> Result<Self, Self::Error> {
        Ok(match int {
            0 => Self::SocketTypeDefault,
            4 => Self::Ipv4,
            41 => Self::Ipv6,
            6 => Self::Tcp,
            17 => Self::Udp,
            1 => Self::Icmp,
            int => return Err(ConversionError::BadInt(int)),
        })
    }
}

// macro_rules! int_enum {
//     ($enm: ident, $cty: ident, $($var: ident),+, $($int: expr),+) => {
//         pub enum $enm {
//             $($var)+,
//         }
//
//         impl From<$enm> for $cty {
//             fn from(enm: $enm) -> Self {
//                 match enm {
//                     $($var => $int),+
//                 }
//             }
//         }
//     };
// }

// A must be one of the sockaddr_* c types
#[repr(C)]
pub struct BindSocketToAddr<A> {
    sockfd: c_int,
    address: A,
}

impl<A: Into<SockAddrIn>> BindSocketToAddr<A> {
    pub fn new(sockfd: c_int, address: A) -> Self {
        Self { sockfd, address }
    }

    pub fn bind(self) -> c_int {
        unsafe {
            bind(
                self.sockfd,
                &self.address as *const A as *const SockAddr,
                SockAddrIn::SIZE,
            )
        }
    }
}

// A must be one of the sockaddr_* c types
#[repr(C)]
pub struct ConnectSocket<A> {
    sockfd: c_int,
    address: A,
}

impl<A: Into<SockAddrIn>> ConnectSocket<A> {
    pub fn new(sockfd: c_int, address: A) -> Self {
        Self { sockfd, address }
    }

    pub fn connect(self) -> c_int {
        unsafe {
            connect(
                self.sockfd,
                &self.address as *const A as *const SockAddr,
                SockAddrIn::SIZE,
            )
        }
    }
}

// A must be one of the sockaddr_* c types
#[repr(C)]
pub struct ListenOnSocket {
    sockfd: c_int,
    backlog: i32,
}

impl ListenOnSocket {
    pub fn new(sockfd: c_int, backlog: i32) -> Self {
        Self { sockfd, backlog }
    }

    pub fn listen(self) -> c_int {
        unsafe { listen(self.sockfd, self.backlog) }
    }
}

#[repr(C)]
pub struct linger {
    // bool active or not
    l_onoff: i32,
    // in seconds
    l_linger: i32,
}

impl linger {
    pub fn new(active: bool, duration: i32) -> Self {
        Self {
            l_onoff: if active { 1 } else { 0 },
            l_linger: duration,
        }
    }
}
