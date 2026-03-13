use pheasant_sys::*;

// A must be one of the sockaddr_* c types
#[repr(C)]
pub struct BindSockToAddr<A: Into<SockAddr>> {
    sockfd: c_int,
    address: A,
}

impl<A: Into<SockAddr>> BindSockToAddr<A> {
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
