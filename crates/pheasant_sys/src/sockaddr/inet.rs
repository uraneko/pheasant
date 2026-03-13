use super::{in_addr_t, sa_family_t};
use crate::{AddressFamily, inet_addr};
use core::ffi::CStr;

// in addr
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: in_addr_t,
}

impl core::fmt::Debug for InAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let a = self.s_addr.to_le_bytes();

        write!(f, "{}.{}.{}.{}", a[0], a[1], a[2], a[3])
    }
}

impl InAddr {
    fn new(o0: u8, o1: u8, o2: u8, o3: u8) -> Self {
        Self {
            s_addr: u32::from_be_bytes([o0, o1, o2, o3]),
        }
    }
}

impl From<[u8; 4]> for InAddr {
    fn from(arr: [u8; 4]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3])
    }
}

impl From<(u8, u8, u8, u8)> for InAddr {
    fn from(tuple: (u8, u8, u8, u8)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2, tuple.3)
    }
}

impl From<&CStr> for InAddr {
    fn from(cstr: &CStr) -> Self {
        Self {
            s_addr: unsafe { inet_addr(cstr.as_ptr()) },
        }
    }
}

// sock addr in
#[allow(non_camel_case_types)]
pub type uint16_t = u16;
// alias of uint16_t
#[allow(non_camel_case_types)]
pub type in_port_t = uint16_t;

// definition fetched from netinet/in.h
// socket address struct for AF_INET address family
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SockAddrIn {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: InAddr,
    // size_of SockAddrIn is 12 octets
    // while size_of SockAddr is 20 octets
    // this padding is needed for the pointer cast
    // NOTE copied this from the libc crate impl
    pub padding: [u8; 8],
}

impl SockAddrIn {
    pub const SIZE: u32 = core::mem::size_of::<SockAddrIn>() as u32;

    pub fn new(af: AddressFamily, addr: impl Into<InAddr>, sin_port: in_port_t) -> Self {
        Self {
            sin_family: af.into(),
            sin_addr: addr.into(),
            sin_port,
            padding: [0; 8],
        }
    }
}
