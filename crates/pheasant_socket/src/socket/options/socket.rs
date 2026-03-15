use super::linger;
use crate::socket::VoidCasting;
use crate::{Error, SocketLevel};
use pheasant_sys::socket::{getsockopt, setsockopt};

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

impl From<SocketOption> for i32 {
    fn from(so: SocketOption) -> i32 {
        use SocketOption::*;
        match so {
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

#[derive(Debug, Clone, Copy)]
pub struct SetSockOpts {
    sockfd: u32,
}

impl SetSockOpts {
    pub fn new(sockfd: u32) -> Self {
        Self { sockfd }
    }

    pub fn snd_buf_size(self, buf_size: i32) -> Result<Self, Error> {
        let ptr = buf_size.cast_ref();
        let len = buf_size.size_of();
        match unsafe {
            setsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into(),
                SocketOption::SndBuf.into(),
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
                SocketLevel::Socket.into(),
                SocketOption::RcvBuf.into(),
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
                SocketLevel::Socket.into(),
                SocketOption::ReuseAddr.into(),
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
                SocketLevel::Socket.into(),
                SocketOption::ReusePort.into(),
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
                SocketLevel::Socket.into(),
                SocketOption::Linger.into(),
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
                SocketLevel::Socket.into(),
                SocketOption::KeepAlive.into(),
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
    pub fn new(sockfd: u32) -> Self {
        Self { sockfd }
    }

    pub fn snd_buf_size(self) -> Result<u32, Error> {
        let mut buf_size = 0;
        let mut size = buf_size.size_of();
        match unsafe {
            getsockopt(
                self.sockfd as i32,
                SocketLevel::Socket.into(),
                SocketOption::SndBuf.into(),
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
                SocketLevel::Socket.into(),
                SocketOption::RcvBuf.into(),
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
                SocketLevel::Socket.into(),
                SocketOption::ReuseAddr.into(),
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
                SocketLevel::Socket.into(),
                SocketOption::ReusePort.into(),
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
                SocketLevel::Socket.into(),
                SocketOption::Linger.into(),
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
                SocketLevel::Socket.into(),
                SocketOption::KeepAlive.into(),
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
