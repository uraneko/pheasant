use crate::AddressFamily;

// in addr
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct InAddr {
    pub addr: u32,
}

impl InAddr {
    pub fn new(o0: u8, o1: u8, o2: u8, o3: u8) -> Self {
        Self {
            addr: u32::from_be_bytes([o3, o2, o1, o0]),
        }
    }

    /// returns the 4 bytes of the address
    pub fn to_bytes(&self) -> [u8; 4] {
        self.addr.to_ne_bytes()
    }

    pub fn any() -> Self {
        Self { addr: 0 }
    }

    pub fn localhost() -> Self {
        Self::new(127, 0, 0, 1)
    }

    // loopback range is last 8 bytes so 127.0.0.x are all loopback addresses
    pub fn loopback(o: u8) -> Self {
        Self::new(127, 0, 0, o)
    }
}

impl core::fmt::Debug for InAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let a = self.addr.to_ne_bytes();

        write!(f, "{}.{}.{}.{}", a[0], a[1], a[2], a[3])
    }
}

pub enum ConversionError {
    StrParseFailed,
    InvalidStr,
}

fn split_str_addr(s: &str) -> Result<[u8; 4], ConversionError> {
    let mut iter = s.split(".").map(|o| o.parse());
    let o0 = iter
        .next()
        .ok_or_else(|| ConversionError::InvalidStr)?
        .map_err(|_| ConversionError::StrParseFailed)?;
    let o1 = iter
        .next()
        .ok_or_else(|| ConversionError::InvalidStr)?
        .map_err(|_| ConversionError::StrParseFailed)?;
    let o2 = iter
        .next()
        .ok_or_else(|| ConversionError::InvalidStr)?
        .map_err(|_| ConversionError::StrParseFailed)?;
    let o3 = iter
        .next()
        .ok_or_else(|| ConversionError::InvalidStr)?
        .map_err(|_| ConversionError::StrParseFailed)?;

    Ok([o0, o1, o2, o3])
}

impl<'a> TryFrom<&'a [u8]> for InAddr {
    type Error = ConversionError;

    fn try_from(s: &'a [u8]) -> Result<Self, Self::Error> {
        str::from_utf8(s)
            .map_err(|_| Self::Error::InvalidStr)?
            .parse()
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

impl core::fmt::Debug for SockAddrIn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl core::fmt::Display for SockAddrIn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let [o0, o1, o2, o3] = self.addr_bytes();
        write!(
            f,
            "inet-addr {{\n  family: {:?},\n  address: {}.{}.{}.{},\n  port: {},\n}}",
            AddressFamily::Inet,
            o0,
            o1,
            o2,
            o3,
            self.port()
        )
    }
}

impl PartialEq for SockAddrIn {
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port && self.addr == other.addr
    }
}

// definition fetched from netinet/in.h
// socket address struct for AF_INET address family
#[repr(C)]
#[derive(Clone, Copy, Eq, Hash)]
pub struct SockAddrIn {
    pub family: u16,
    pub port: u16,
    pub addr: InAddr,
    pub padding: [u8; PADDING],
}
pub const PADDING: usize = (super::SockAddr::SIZE as usize) - core::mem::size_of::<InAddr>() - 4;

impl Default for SockAddrIn {
    fn default() -> Self {
        Self {
            family: Self::AF.into(),
            port: 0,
            addr: InAddr::default(),
            padding: [0u8; _],
        }
    }
}

impl SockAddrIn {
    pub const SIZE: u32 = core::mem::size_of::<SockAddrIn>() as u32;
    pub const AF: AddressFamily = AddressFamily::Inet;

    pub fn new(addr: impl Into<InAddr>, port: u16) -> Self {
        let port = u16::from_be(port);
        Self {
            family: Self::AF.into(),
            addr: addr.into(),
            port,
            padding: [0; _],
        }
    }

    /// returns the 4 bytes of the address in native endianne order
    pub fn addr_bytes(&self) -> [u8; 4] {
        self.addr.to_bytes()
    }

    /// returns the port value of this address
    pub fn port(&self) -> u16 {
        u16::from_be(self.port)
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
        let addr: InAddr = addr_str.parse()?;

        // TODO user could not specify a port num
        // in fact thats likely the most prevalent use
        let Some(port_str) = iter.next() else {
            return Err(Self::Err::InvalidStr);
        };
        let port = port_str.parse().map_err(|_| Self::Err::StrParseFailed)?;

        Ok(Self::new(addr, port))
    }
}

impl crate::socket::SockAddrCasting for SockAddrIn {
    const ADDRESS_FAMILY: crate::AddressFamily = Self::AF;
}
impl crate::socket::TrueSockAddr for SockAddrIn {}
