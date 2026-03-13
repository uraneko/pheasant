use crate::Error;
use crate::{AddressFamily, inet_addr};

// in addr
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

impl InAddr {
    pub fn to_bytes(&self) -> [u8; 4] {
        self.s_addr.to_le_bytes()
    }
}

impl core::fmt::Debug for InAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let a = self.s_addr.to_le_bytes();

        write!(f, "{}.{}.{}.{}", a[0], a[1], a[2], a[3])
    }
}

pub enum ConversionError {
    StrParseFailed,
    InvalidStr,
}

fn split_str_addr(s: &str) -> Result<[u8; 4], ConversionError> {
    let mut iter = s.split(".").map(|b| b.parse());
    let b0 = iter
        .next()
        .ok_or_else(|| ConversionError::InvalidStr)?
        .map_err(|_| ConversionError::StrParseFailed)?;
    let b1 = iter
        .next()
        .ok_or_else(|| ConversionError::InvalidStr)?
        .map_err(|_| ConversionError::StrParseFailed)?;
    let b2 = iter
        .next()
        .ok_or_else(|| ConversionError::InvalidStr)?
        .map_err(|_| ConversionError::StrParseFailed)?;
    let b3 = iter
        .next()
        .ok_or_else(|| ConversionError::InvalidStr)?
        .map_err(|_| ConversionError::StrParseFailed)?;

    Ok([b0, b1, b2, b3])
}

impl InAddr {
    fn new(o0: u8, o1: u8, o2: u8, o3: u8) -> Self {
        Self {
            s_addr: u32::from_be_bytes([o0, o1, o2, o3]),
        }
    }
}

impl<'a> TryFrom<&'a str> for InAddr {
    type Error = ConversionError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl core::str::FromStr for InAddr {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [o0, o1, o2, o3] = split_str_addr(s)?;

        Ok(Self::new(o0, o1, o2, o3))
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

// definition fetched from netinet/in.h
// socket address struct for AF_INET address family
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SockAddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    // size_of SockAddrIn is 12 octets
    // while size_of SockAddr is 20 octets
    // this padding is needed for the pointer cast
    // NOTE copied this from the libc crate
    pub padding: [u8; 8],
}

impl SockAddrIn {
    pub const SIZE: u32 = core::mem::size_of::<SockAddrIn>() as u32;
    pub const AF: AddressFamily = AddressFamily::AfInet;

    pub fn new(addr: impl Into<InAddr>, sin_port: u16) -> Self {
        let sin_port = u16::from_be(sin_port);
        Self {
            sin_family: Self::AF.into(),
            sin_addr: addr.into(),
            sin_port,
            padding: [0; 8],
        }
    }
}

impl<'a> TryFrom<&'a str> for SockAddrIn {
    type Error = ConversionError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl core::str::FromStr for SockAddrIn {
    type Err = ConversionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(":");

        let Some(addr_str) = iter.next() else {
            return Err(Self::Err::InvalidStr);
        };
        let sin_addr = addr_str.parse()?;

        let Some(port_str) = iter.next() else {
            return Err(Self::Err::InvalidStr);
        };
        let sin_port = port_str.parse().map_err(|_| Self::Err::StrParseFailed)?;
        let sin_port = u16::from_be(sin_port);

        Ok(Self {
            sin_family: Self::AF.into(),
            sin_addr,
            sin_port,
            padding: [0; 8],
        })
    }
}
