use super::*;
use core::ffi::c_int;

// A must be one of the sockaddr_* c types
#[repr(C)]
pub struct Listen {
    // fd of the to be server socket
    sockfd: c_int,
    // backlog size of the to be server socket
    backlog: i32,
}

impl Listen {
    pub fn new(sockfd: c_int, backlog: i32) -> Self {
        Self { sockfd, backlog }
    }

    pub fn listen(self) -> Result<(), Error> {
        match unsafe { listen(self.sockfd, self.backlog) } {
            0 => Ok(()),
            err => Err(err.into()),
        }
    }
}
