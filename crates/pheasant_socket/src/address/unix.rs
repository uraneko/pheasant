use crate::AddressFamily;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SockAddrUn {
    // AF_UNIX
    sun_family: u16,
    // suffix with \0 for best interoperability
    // \0 being included in the 108  bytes
    sun_path: [u8; 108],
}

/// there are 3 different approaches to initializing a unix socket
/// - pathname: using a path in the local fs
/// - unnamed: not binding to any path, just a pathless / nameless socket
/// - abstract: a path str that starts with a null byte and doesn't represent a
/// socket file in the local fs
impl SockAddrUn {
    pub const SIZE: u32 = core::mem::size_of::<Self>() as u32;
    pub const AF: AddressFamily = AddressFamily::AfUnix;

    pub fn pathname_unchecked(arr: [u8; 108]) -> Self {
        Self {
            sun_family: Self::AF.into(),
            sun_path: arr,
        }
    }

    pub fn abstract_unchecked(arr: [u8; 108]) -> Self {
        Self {
            sun_family: Self::AF.into(),
            sun_path: arr,
        }
    }

    pub fn unnamed() -> Self {
        Self {
            sun_family: Self::AF.into(),
            sun_path: [0; _],
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
        if bytes.len() > 108 {
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
