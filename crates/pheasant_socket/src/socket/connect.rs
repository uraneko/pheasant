use pheasant_sys::*;

// A must be one of the sockaddr_* c types
#[repr(C)]
pub struct ConnectSock<A> {
    sockfd: c_int,
    address: A,
}

impl<A: Into<SockAddrIn>> ConnectSock<A> {
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
