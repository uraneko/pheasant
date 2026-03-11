#![no_std]
#![allow(non_camel_case_types)]
//! socket domain and type definitions can be found at
//! /usr/include/bits/socket.h
//! #include <sys/socket.h>
//! #include <netinet/in.h>
//! #include <arpa/inet.h>

// TODO the c bindings and definitions should go into a new crate pheasant_sys
// while the externally exposable rust safe socket api should go into a new pheasant_socket
// also rename pheasant_http to pheasant_prologue

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
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;

    pub fn read(sockfd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;

    pub fn write(sockfd: c_int, buf: *const c_void, count: size_t) -> ssize_t;

    pub fn inet_addr(addr: *const i8) -> in_addr_t;
}

type size_t = u64;
type ssize_t = u64;

#[repr(C)]
pub enum SocketLevel {
    SolSocket = 0,
    IprotoTcp = 6,
}

impl SocketLevel {
    pub fn as_int(&self) -> c_int {
        use SocketLevel::*;
        match self {
            IPPROTO_IP => 0,
            IPPROTO_ICMP => 1,
            IPPROTO_IGMP => 2,
            IPPROTO_IPIP => 4,
            IPPROTO_TCP => 6,
            IPPROTO_EGP => 8,
            IPPROTO_PUP => 12,
            IPPROTO_UDP => 17,
            IPPROTO_IDP => 22,
            IPPROTO_TP => 29,
            IPPROTO_DCCP => 33,
            IPPROTO_IPV6 => 41,
            IPPROTO_RSVP => 46,
            IPPROTO_GRE => 47,
            IPPROTO_ESP => 50,
            IPPROTO_AH => 51,
            IPPROTO_MTP => 92,
            IPPROTO_BEETPH => 94,
            IPPROTO_ENCAP => 98,
            IPPROTO_PIM => 103,
        }
    }
}

#[repr(C)]
pub enum SocketOption {
    SO_DEBUG = 1,
    SO_BROADCAST = 6,
    SO_REUSEADDR = 2,
    SO_REUSEPORT = 15,
    SO_KEEPALIVE = 9,
    SO_LINGER = 13,
    SO_OOBINLINE = 10,
    SO_SNDBUF = 7,
    SO_RCVBUF = 8,
    SO_DONTROUTE = 5,
    SO_RCVLOWAT = 18,
    SO_RCVTIMEO = 66,
    SO_SNDLOWAT = 19,
    SO_SNDTIMEO = 67,
}

impl SocketOption {
    pub fn as_int(&self) -> c_int {
        use SocketOption::*;
        match self {
            SO_DEBUG => 1,
            SO_BROADCAST => 6,
            SO_REUSEADDR => 2,
            SO_REUSEPORT => 15,
            SO_KEEPALIVE => 9,
            SO_LINGER => 13,
            SO_OOBINLINE => 10,
            SO_SNDBUF => 7,
            SO_RCVBUF => 8,
            SO_DONTROUTE => 5,
            SO_RCVLOWAT => 18,
            SO_RCVTIMEO => 66,
            SO_SNDLOWAT => 19,
            SO_SNDTIMEO => 67,
        }
    }
}

type socklen_t = c_uint;

// TODO
unsafe extern "C" {
    pub fn perror(msg: *const u8);

    pub fn strerror();

    pub fn ioctl();

    pub fn fcntl();
}

pub struct AcquireSockFd {
    domain: ProtocolFamily,
    type_: SocketType,
    proto: ProtocolNumber,
}

impl From<c_int> for ProtocolNumber {
    fn from(int: c_int) -> Self {
        if int == 0 {
            return Default::default();
        }

        panic!("this should be a TryInto impl");
    }
}

impl AcquireSockFd {
    pub fn new(
        domain: ProtocolFamily,
        type_: SocketType,
        proto: impl Into<ProtocolNumber>,
    ) -> Self {
        Self {
            domain,
            type_,
            proto: proto.into(),
        }
    }

    pub fn acquire(self) -> c_int {
        unsafe {
            socket(
                self.domain.as_int(),
                self.type_.as_int(),
                self.proto.as_int(),
            )
        }
    }
}

// values gotten from /usr/include/bits/socket.h
// under /* Protocol families.  */ definitions
// AF_* definitions are just aliases for the PF_* definitions there
pub enum ProtocolFamily {
    // for local communications
    PfUnix, // aka AF_LOCAL
    // for ipv4
    PfInet,
    // for ipv6
    PfInet6,
    PfIpx,
    PfNetlinK,
    PfX25,
    PfAx25,
    PfAtmpvc,
    PfAppletalk,
    PfPacket,
}

pub type AddressFamily = ProtocolFamily;

impl ProtocolFamily {
    pub fn as_int(&self) -> c_int {
        use ProtocolFamily::*;

        match self {
            PfUnix => 1,
            PfInet => 2,
            PfInet6 => 10,
            PfIpx => 4,
            PfNetlinK => 16,
            PfX25 => 9,
            PfAx25 => 3,
            PfAtmpvc => 8,
            PfAppletalk => 5,
            PfPacket => 17,
        }
    }
}

// values retrieved from bits/socket_type.h
// under section /* Types of sockets.  */
pub enum SocketType {
    // use this to open a tcp socket
    SockStream,
    // use this to open a udp socket
    SockDgram,
    SockSeqPacket,
    // use this for direct access to the underlying ip protocol
    SockRaw,
    SockRdm,
    SockPacket,
}

impl SocketType {
    pub fn as_int(&self) -> c_int {
        use SocketType::*;

        match self {
            SockStream => 1,
            SockDgram => 2,
            SockRaw => 3,
            SockRdm => 4,
            SockSeqPacket => 5,
            SockPacket => 10,
        }
    }
}

#[derive(Default)]
pub enum ProtocolNumber {
    #[default]
    SocketTypeDefault,
    Ipv4,
    Ipv6,
    Tcp,
    Udp,
    Else(c_int),
}

impl ProtocolNumber {
    pub fn as_int(&self) -> c_int {
        match self {
            Self::SocketTypeDefault => 0,
            Self::Ipv4 => 4,
            Self::Ipv6 => 41,
            Self::Tcp => 6,
            Self::Udp => 17,
            Self::Else(num) => *num,
        }
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
