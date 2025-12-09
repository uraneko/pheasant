use crate::PheasantError;
use crate::{ByteIterator, ClientError, ErrorStatus, ServerError, err_stt};
use alloc::str::FromStr;
use alloc::string::String;
use core::fmt;
use proc_macro2::{Delimiter, Group, Span, TokenStream as TS2, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::Ident;

/// HTTP Method enum
/// only Get method is somewhat supported at the moment
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Method {
    Head = 1,
    Get = 2,
    Post = 4,
    Put = 8,
    Patch = 16,
    Delete = 32,
    Connect = 64,
    Options = 128,
    Trace = 256,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Head => "HEAD",
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
            Self::Connect => "CONNECT",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
        }
    }
}

impl TryFrom<&[u8]> for Method {
    type Error = PheasantError;

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        match s {
            b"HEAD" | b"head" => Ok(Self::Head),
            b"GET" | b"get" => Ok(Self::Get),
            b"POST" | b"post" => Ok(Self::Post),
            b"PUT" | b"put" => Ok(Self::Put),
            b"PATCH" | b"patch" => Ok(Self::Patch),
            b"DELETE" | b"delete" => Ok(Self::Delete),
            b"CONNECT" | b"connect" => Ok(Self::Connect),
            b"OPTIONS" | b"options" => Ok(Self::Options),
            b"TRACE" | b"trace" => Ok(Self::Trace),
            _ => Err(Self::Error::ClientError(ClientError::BadRequest)),
        }
    }
}

impl FromStr for Method {
    type Err = ErrorStatus;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HEAD" | "head" => Ok(Self::Head),
            "GET" | "get" => Ok(Self::Get),
            "POST" | "post" => Ok(Self::Post),
            "PUT" | "put" => Ok(Self::Put),
            "PATCH" | "patch" => Ok(Self::Patch),
            "DELETE" | "delete" => Ok(Self::Delete),
            "CONNECT" | "connect" => Ok(Self::Connect),
            "OPTIONS" | "options" => Ok(Self::Options),
            "TRACE" | "trace" => Ok(Self::Trace),
            _ => err_stt!(?BadRequest),
        }
    }
}

impl Method {
    pub fn from_iter<I: Iterator<Item = u8>>(i: I) -> Result<Self, ErrorStatus> {
        ByteIterator::new(i).try_into()
    }
}

impl<I> TryFrom<ByteIterator<I>> for Method
where
    I: Iterator<Item = u8>,
{
    type Error = ErrorStatus;

    fn try_from(iter: ByteIterator<I>) -> Result<Self, Self::Error> {
        let mut iter = iter.iter;
        while let Some(num) = iter.next() {
            match num {
                b'G' => try_method_get(&mut iter)?,
                b'H' => try_method_head(&mut iter)?,
                b'P' => try_method_p_(&mut iter)?,
                b'O' => try_method_opts(&mut iter)?,
                b'D' => try_method_del(&mut iter)?,
                b'C' => try_method_conn(&mut iter)?,
                b'T' => try_method_trc(&mut iter)?,
                _ => return Err(ErrorStatus::Server(ServerError::NotImplemented)),
            }
        }

        Err(ErrorStatus::Client(ClientError::BadRequest))
    }
}

fn try_method_get<I>(iter: &mut I) -> Result<(), ErrorStatus>
where
    I: Iterator<Item = u8>,
{
    let mut idx = 1;
    while let Some(ch) = iter.next() {
        match [idx, ch] {
            [1, b'E'] => idx += 1,
            [2, b'T'] => return Ok(()),
            _ => return Err(ErrorStatus::Server(ServerError::NotImplemented)),
        }
    }

    Err(ErrorStatus::Client(ClientError::BadRequest))
}

fn try_method_head<I>(iter: &mut I) -> Result<(), ErrorStatus>
where
    I: Iterator<Item = u8>,
{
    let mut idx = 1;
    while let Some(ch) = iter.next() {
        match [idx, ch] {
            [1, b'E'] | [2, b'A'] => idx += 1,
            [3, b'D'] => return Ok(()),
            _ => return Err(ErrorStatus::Server(ServerError::NotImplemented)),
        }
    }

    Err(ErrorStatus::Client(ClientError::BadRequest))
}

fn try_method_opts<I>(iter: &mut I) -> Result<(), ErrorStatus>
where
    I: Iterator<Item = u8>,
{
    let mut idx = 1;
    while let Some(ch) = iter.next() {
        match [idx, ch] {
            [1, b'P'] | [2, b'T'] | [3, b'I'] | [4, b'O'] | [5, b'N'] => idx += 1,
            [6, b'S'] => return Ok(()),
            _ => return Err(ErrorStatus::Server(ServerError::NotImplemented)),
        }
    }

    Err(ErrorStatus::Client(ClientError::BadRequest))
}

fn try_method_del<I>(iter: &mut I) -> Result<(), ErrorStatus>
where
    I: Iterator<Item = u8>,
{
    let mut idx = 1;
    while let Some(ch) = iter.next() {
        match [idx, ch] {
            [1, b'E'] | [2, b'L'] | [3, b'E'] | [4, b'T'] => idx += 1,
            [5, b'E'] => return Ok(()),
            _ => return Err(ErrorStatus::Server(ServerError::NotImplemented)),
        }
    }

    Err(ErrorStatus::Client(ClientError::BadRequest))
}

fn try_method_trc<I>(iter: &mut I) -> Result<(), ErrorStatus>
where
    I: Iterator<Item = u8>,
{
    let mut idx = 1;
    while let Some(ch) = iter.next() {
        match [idx, ch] {
            [1, b'R'] | [2, b'A'] | [3, b'C'] => idx += 1,
            [4, b'E'] => return Ok(()),
            _ => return Err(ErrorStatus::Server(ServerError::NotImplemented)),
        }
    }

    Err(ErrorStatus::Client(ClientError::BadRequest))
}

fn try_method_conn<I>(iter: &mut I) -> Result<(), ErrorStatus>
where
    I: Iterator<Item = u8>,
{
    let mut idx = 1;
    while let Some(ch) = iter.next() {
        match [idx, ch] {
            [1, b'O'] | [2, b'N'] | [3, b'N'] | [4, b'E'] | [5, b'C'] => idx += 1,
            [6, b'T'] => return Ok(()),
            _ => return Err(ErrorStatus::Server(ServerError::NotImplemented)),
        }
    }

    Err(ErrorStatus::Client(ClientError::BadRequest))
}

fn try_method_p_<I>(iter: &mut I) -> Result<(), ErrorStatus>
where
    I: Iterator<Item = u8>,
{
    match iter.next() {
        Some(b'O') => try_method_post(iter)?,
        Some(b'U') => try_method_put(iter)?,
        Some(b'A') => try_method_patch(iter)?,
        _ => return Err(ErrorStatus::Server(ServerError::NotImplemented)),
    }

    // in this case we have a method named P
    // and not an empty method
    // so we return not implemented instead of bad request
    return Err(ErrorStatus::Server(ServerError::NotImplemented));
}

fn try_method_post<I>(iter: &mut I) -> Result<(), ErrorStatus>
where
    I: Iterator<Item = u8>,
{
    let mut idx = 2;
    while let Some(ch) = iter.next() {
        match [idx, ch] {
            [2, b'S'] => idx += 1,
            [3, b'T'] => return Ok(()),
            _ => return Err(ErrorStatus::Server(ServerError::NotImplemented)),
        }
    }

    return Err(ErrorStatus::Server(ServerError::NotImplemented));
}

fn try_method_put<I>(iter: &mut I) -> Result<(), ErrorStatus>
where
    I: Iterator<Item = u8>,
{
    match iter.next() {
        Some(b'T') => return Ok(()),
        _ => return Err(ErrorStatus::Server(ServerError::NotImplemented)),
    }
}

fn try_method_patch<I>(iter: &mut I) -> Result<(), ErrorStatus>
where
    I: Iterator<Item = u8>,
{
    let mut idx = 2;
    while let Some(ch) = iter.next() {
        match [idx, ch] {
            [2, b'T'] | [3, b'C'] => idx += 1,
            [4, b'H'] => return Ok(()),
            _ => return Err(ErrorStatus::Server(ServerError::NotImplemented)),
        }
    }

    return Err(ErrorStatus::Server(ServerError::NotImplemented));
}

impl TryFrom<String> for Method {
    type Error = ErrorStatus;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}
