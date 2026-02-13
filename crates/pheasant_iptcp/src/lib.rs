//! socket domain and type definitions can be found at /usr/include/bits/socket.h
// #include <sys/socket.h>
// #include <netinet/in.h>
// #include <arpa/inet.h>

type int = i16;

// #include <sys/socket.h>
unsafe extern "C" {
    pub fn socket(domain: int, type_: int, protocol: int) -> int;

    pub fn setsockopt(
        fd: int,
        level: int,
        optname: int,
        // optval: *const std::ffi::c_void,
        optval: *const u32,
        optlen: socketlen_t,
    ) -> int;

    pub fn getsockopt(
        fd: int,
        level: int,
        optname: int,
        optval: *mut i32,
        optlen: socketlen_t,
    ) -> int;

    pub fn bind(fd: int, addr: *mut sockaddr, addrlen: socketlen_t);
}

unsafe extern "C" {
    pub fn perror(msg: *const u8);
}

type socketlen_t = u32;
type sa_family_t = u32;

#[repr(C)]
pub struct sockaddr_storage {
    ss_family: sa_family_t, /* Address family */
}

#[repr(C)]
pub struct sockaddr {
    sa_family: sa_family_t,
    sa_data: [u8; 14],
}

struct in_addr {
    s_addr: in_addr_t,
}

type in_addr_t = u32;
type in_port_t = u16;

struct sockaddr_in {
    sin_family: sa_family_t, /* AF_INET */
    sin_port: in_port_t,     /* Port number */
    sin_addr: in_addr,       /* IPv4 address */
}

pub struct sockaddr_un {
    pub sun_family: sa_family_t, /* Address family */
    pub sun_path: Vec<u8>,       /* Socket pathname */
}

pub const AF_UNIX: u32 = 1;
pub const SOL_SOCKET: i16 = 1;
pub const SO_REUSEADDR: i16 = 2;
pub const SO_REUSEPORT: i16 = 15;
pub const SO_ERROR: i16 = 4;
pub const OPT: i16 = 1;

#[repr(C)]
pub enum Domain {
    // ipv4 addr
    AfInet = 2,
    // local addr, such as file:///... if i understand correctly
    AfUnix = 1,
}

impl From<Domain> for i16 {
    fn from(domain: Domain) -> i16 {
        match domain {
            Domain::AfInet => 2,
            Domain::AfUnix => 1,
        }
    }
}

#[repr(C)]
pub enum Type {
    // for tcp based communication
    SockStream = 1,
    // for udp based communication
    SockDgram = 2,
}

impl From<Type> for i16 {
    fn from(ty: Type) -> i16 {
        match ty {
            Type::SockStream => 1,
            Type::SockDgram => 2,
        }
    }
}
