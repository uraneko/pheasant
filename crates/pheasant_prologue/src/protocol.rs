use crate::{ByteIterator, ClientError, ErrorStatus, PheasantError, ServerError, repeat_tfs};
use alloc::str::FromStr;
use core::fmt::{self, Display, Formatter};
/// Http protocol version
///
/// currently only http 1.1 is supported
#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Protocol {
    #[default]
    Http11 = 1,
    Http2 = 2,
}

impl From<Protocol> for u8 {
    fn from(p: Protocol) -> u8 {
        match p {
            Protocol::Http11 => 1,
            Protocol::Http2 => 2,
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&[u8]> for Protocol {
    type Error = PheasantError;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        match v {
            b"HTTP/1.1" => Ok(Self::Http11),
            b"HTTP/2" | b"HTTP/3" => Err(Self::Error::ServerError(
                ServerError::HTTPVersionNotSupported,
            )),
            _ => Err(Self::Error::ClientError(ClientError::BadRequest)),
        }
    }
}

impl FromStr for Protocol {
    type Err = PheasantError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(Self::Http11),
            "HTTP/2" | "HTTP/3" => {
                Err(Self::Err::ServerError(ServerError::HTTPVersionNotSupported))
            }
            _ => Err(Self::Err::ClientError(ClientError::BadRequest)),
        }
    }
}
repeat_tfs!(Protocol);

impl Protocol {
    pub fn from_iter<I: Iterator<Item = u8>>(i: I) -> Result<Self, ErrorStatus> {
        ByteIterator::new(i).try_into()
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Http11 => "HTTP/1.1",
            Self::Http2 => "HTTP/2",
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl<I> TryFrom<ByteIterator<I>> for Protocol
where
    I: Iterator<Item = u8>,
{
    type Error = ErrorStatus;

    fn try_from(iter: ByteIterator<I>) -> Result<Self, Self::Error> {
        let mut iter = iter.iter;
        while let Some(num) = iter.next() {
            match num {
                b'H' => {
                    let _is_http = try_proto_http(&mut iter)?;
                    match iter.next() {
                        Some(b'0') => {
                            return Err(ErrorStatus::Server(ServerError::HTTPVersionNotSupported));
                        }
                        Some(b'1') => try_ver_1_(&mut iter)?,
                        Some(b'2') => try_ver_done(&mut iter)?,
                        Some(b'3') => try_ver_done(&mut iter)?,
                        // or maybe return not implemented
                        _ => return Err(ErrorStatus::Client(ClientError::BadRequest)),
                    }
                }
                _ => return Err(ErrorStatus::Client(ClientError::BadRequest)),
            }
        }

        Err(ErrorStatus::Client(ClientError::BadRequest))
    }
}

fn try_proto_http<I>(iter: &mut I) -> Result<(), ErrorStatus>
where
    I: Iterator<Item = u8>,
{
    let mut idx = 1;
    while let Some(ch) = iter.next() {
        match [idx, ch] {
            [1, b'T'] | [2, b'T'] | [3, b'P'] => idx += 1,
            [4, b'/'] => return Ok(()),
            _ => return Err(ErrorStatus::Client(ClientError::BadRequest)),
        }
    }

    Err(ErrorStatus::Client(ClientError::BadRequest))
}

fn try_ver_1_<I>(iter: &mut I) -> Result<(), ErrorStatus>
where
    I: Iterator<Item = u8>,
{
    let Some(b'.') = iter.next() else {
        return Err(ErrorStatus::Client(ClientError::BadRequest));
    };

    match iter.next() {
        Some(b'1') => (),
        Some(b'0') => return Err(ErrorStatus::Server(ServerError::HTTPVersionNotSupported)),
        _ => return Err(ErrorStatus::Client(ClientError::BadRequest)),
    }

    if iter.next().is_some() {
        Err(ErrorStatus::Client(ClientError::BadRequest))
    } else {
        Ok(())
    }
}

fn try_ver_done<I>(iter: &mut I) -> Result<(), ErrorStatus>
where
    I: Iterator<Item = u8>,
{
    if iter.next().is_some() {
        Err(ErrorStatus::Client(ClientError::BadRequest))
    } else {
        Err(ErrorStatus::Server(ServerError::HTTPVersionNotSupported))
    }
}
