#[repr(C)]
// this is really what modern documentations call sockaddr_storage
// my understanding is that,
// since the original sockaddr is incapable of holding large addresses such as inet6 or unix
// sockaddr_storage was made to be large enough for all sockaddr_* variants
/// never use this directly
/// this is not aa user facing struct
pub struct SockAddr {
    family: u16,
    data: [u8; 110],
}

impl SockAddr {
    pub const SIZE: u32 = core::mem::size_of::<Self>() as u32;
}

use core::ffi::{c_int, c_uint, c_void};

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

    pub fn recv(sockfd: c_int, buf: *mut c_void, count: size_t, flags: c_int) -> i64;

    // the return type is ssize_t
    // i had thought ssize_t = u64 but on error this and recv return -1 and set errno
    pub fn send(sockfd: c_int, buf: *const c_void, count: size_t, flags: c_int) -> i64;

    // WARN dont use
    /// dont use this function
    ///
    /// use inet_aton() / inet_pton() instead
    // #[deprecated]
    // WARN this function's return values in cases of error/success make it problematic
    // hence it's deprecated
    // pub fn inet_addr(addr: *const i8) -> in_addr_t;

    // pub fn inet_aton();
    // pub fn inet_pton();

    //
    pub fn shutdown(sockfd: c_int, how: c_int) -> c_int;

    pub fn close(fd: c_int) -> c_int;

    pub fn unlink(pathname: *const u8) -> i32;

    // TODO
    fn socketpair();

    // TODO
    fn getaddrinfo();

    // TODO
    fn poll();

    // TODO
    fn select();
}

type size_t = u64;
type ssize_t = u64;

type socklen_t = c_uint;

// TODO
unsafe extern "C" {
    pub fn perror(msg: *const u8);

    // TODO
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;

    // TODO
    pub fn ioctl(d: c_int, request: c_int, ...) -> c_int;
}
