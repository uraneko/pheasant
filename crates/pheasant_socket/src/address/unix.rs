use crate::AddressFamily;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SockAddrUn {
    // AF_UNIX
    family: u16,
    // suffix with \0 for best interoperability
    // \0 being included in the 108  bytes
    path: [u8; 108],
    padding: [u8; PADDING],
}
const PADDING: usize = super::SockAddr::SIZE as usize - 110;

/// there are 3 different approaches to initializing a unix socket
/// - pathname: using a path in the local fs
/// - unnamed: not binding to any path, just a pathless / nameless socket
/// - abstract: a path str that starts with a null byte and doesn't represent a
/// socket file in the local fs
impl SockAddrUn {
    pub const SIZE: u32 = core::mem::size_of::<Self>() as u32;
    pub const AF: AddressFamily = AddressFamily::Unix;

    pub fn path(&self) -> &[u8] {
        let Some(pos) = self.path.iter().position(|ch| *ch == 0) else {
            return &self.path;
        };

        &self.path[..pos]
    }

    pub fn pathname_unchecked(arr: [u8; 108]) -> Self {
        Self {
            family: Self::AF.into(),
            path: arr,
            padding: [0; _],
        }
    }

    pub fn abstract_unchecked(arr: [u8; 108]) -> Self {
        Self {
            family: Self::AF.into(),
            path: arr,
            padding: [0; _],
        }
    }

    pub fn unnamed() -> Self {
        Self {
            family: Self::AF.into(),
            path: [0; _],
            padding: [0; _],
        }
    }
}

pub enum ConversionError {
    PathnameTooLong,
}

impl<'a> TryFrom<&'a str> for SockAddrUn {
    type Error = ConversionError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl core::str::FromStr for SockAddrUn {
    type Err = ConversionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let len = bytes.len();
        if (len == 108 && bytes[107] != 0) || len > 108 {
            return Err(Self::Err::PathnameTooLong);
        }

        let Some(first) = bytes.first() else {
            return Ok(Self::unnamed());
        };
        let bytes = slice_to_array(bytes);

        if *first == 0 {
            return Ok(Self::abstract_unchecked(bytes));
        } else {
            return Ok(Self::pathname_unchecked(bytes));
        }
    }
}

fn slice_to_array(slice: &[u8]) -> [u8; 108] {
    let used = slice.len();
    let mut arr = [0u8; 108];
    for idx in 0..used {
        arr[idx] = slice[idx]
    }

    arr
}

impl crate::socket::SockAddrCasting for SockAddrUn {
    const ADDRESS_FAMILY: crate::AddressFamily = Self::AF;
    const SIZE: u32 = Self::SIZE - 2;
}
impl crate::socket::TrueSockAddr for SockAddrUn {}
