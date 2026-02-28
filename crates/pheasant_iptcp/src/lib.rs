//! socket domain and type definitions can be found at
//! /usr/include/bits/socket.h
//! #include <sys/socket.h>
//! #include <netinet/in.h>
//! #include <arpa/inet.h>

use core::ffi::{c_char, c_int, c_uint};

unsafe extern "C" {
    pub fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;

    pub fn bind(sockfd: c_int, socket_address: SocketAddress, socketlen_t: c_int) -> c_int;

    // socket len is size of socket's address structure type
    pub fn connect(sockfd: c_int, socket_address: SocketAddress, socketlen_t: c_int) -> c_int;
}

// TODO
unsafe extern "C" {
    pub fn perror(msg: *const u8);

    pub fn strerror();

    pub fn ioctl();

    pub fn fcntl();
}

#[allow(non_camel_case_types)]
type sa_family_t = c_uint;

#[repr(C)]
pub struct SocketAddress {
    sa_family: sa_family_t,
    sa_data: [c_char; 14],
}

pub struct AcquireSockFd {
    domain: Domain,
    type_: SocketType,
    proto: ProtocolNumber,
}

impl AcquireSockFd {
    pub fn new(domain: Domain, type_: SocketType, proto: ProtocolNumber) -> Self {
        Self {
            domain,
            type_,
            proto,
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
pub enum Domain {
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

impl Domain {
    pub fn as_int(&self) -> c_int {
        use Domain::*;

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

macro_rules! int_enum {
    ($enm: ident, $cty: ident, $($var: ident),+, $($int: expr),+) => {
        pub enum $enm {
            $($var)+,
        }

        impl From<$enm> for $cty {
            fn from(enm: $enm) -> Self {
                match enm {
                    $($var => $int),+
                }
            }
        }
    };
}
