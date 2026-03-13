use super::*;

#[derive(Debug, Clone, Copy)]
pub struct SetSockOpts {
    sockfd: u32,
}

impl SetSockOpts {
    pub fn snd_buf_size(self, buf_size: i32) -> Result<Self, Error> {
        let ptr = buf_size.cast_ref();
        let len = buf_size.size_of();
        match unsafe {
            setsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into_int(),
                SocketOption::SndBuf.into_int(),
                ptr,
                len,
            )
        } {
            0 => Ok(self),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn rcv_buf_size(self, buf_size: i32) -> Result<Self, Error> {
        let ptr = buf_size.cast_ref();
        let len = buf_size.size_of();
        match unsafe {
            setsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into_int(),
                SocketOption::RcvBuf.into_int(),
                ptr,
                len,
            )
        } {
            0 => Ok(self),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn reuse_address(self, reuse: bool) -> Result<Self, Error> {
        let switch = if reuse { 1 } else { 0 };
        match unsafe {
            setsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into_int(),
                SocketOption::ReuseAddr.into_int(),
                switch.cast_ref(),
                switch.size_of(),
            )
        } {
            0 => Ok(self),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn reuse_port(self, reuse: bool) -> Result<Self, Error> {
        let switch = if reuse { 1 } else { 0 };
        match unsafe {
            setsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into_int(),
                SocketOption::ReusePort.into_int(),
                switch.cast_ref(),
                switch.size_of(),
            )
        } {
            0 => Ok(self),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn linger(self, linger: linger) -> Result<Self, Error> {
        match unsafe {
            setsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into_int(),
                SocketOption::Linger.into_int(),
                linger.cast_ref(),
                linger.size_of(),
            )
        } {
            0 => Ok(self),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn keepalive(self, keep: bool) -> Result<Self, Error> {
        let switch = if keep { 1 } else { 0 };
        match unsafe {
            setsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into_int(),
                SocketOption::KeepAlive.into_int(),
                switch.cast_ref(),
                switch.size_of(),
            )
        } {
            0 => Ok(self),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetSockOpts {
    sockfd: u32,
}

impl GetSockOpts {
    pub fn snd_buf_size(self) -> Result<u32, Error> {
        let mut buf_size = 0;
        let mut size = buf_size.size_of();
        match unsafe {
            getsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into_int(),
                SocketOption::SndBuf.into_int(),
                buf_size.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            0 => Ok(buf_size),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn rcv_buf_size(self) -> Result<u32, Error> {
        let mut buf_size = 0;
        let mut size = buf_size.size_of();
        match unsafe {
            getsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into_int(),
                SocketOption::RcvBuf.into_int(),
                buf_size.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            0 => Ok(buf_size),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn reuse_address(self) -> Result<bool, Error> {
        let mut switch = false;
        let mut size = switch.size_of();
        match unsafe {
            getsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into_int(),
                SocketOption::ReuseAddr.into_int(),
                switch.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            0 => Ok(switch),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn reuse_port(self) -> Result<bool, Error> {
        let mut switch = false;
        let mut size = switch.size_of();
        match unsafe {
            getsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into_int(),
                SocketOption::ReusePort.into_int(),
                switch.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            0 => Ok(switch),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn linger(self) -> Result<linger, Error> {
        let mut linger = linger::new(true, 432);
        let mut size = linger.size_of();
        match unsafe {
            getsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into_int(),
                SocketOption::Linger.into_int(),
                linger.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            0 => Ok(linger),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }

    pub fn keepalive(self) -> Result<bool, Error> {
        let mut switch = false;
        let mut size = switch.size_of();
        match unsafe {
            getsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into_int(),
                SocketOption::KeepAlive.into_int(),
                switch.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            0 => Ok(switch),
            -1 => Err(Error::errno()),
            err => unreachable!("unexpected error code {}", err),
        }
    }
}
