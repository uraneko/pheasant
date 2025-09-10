use crate::PheasantError;
use crate::{ClientError, ErrorStatus, ServerError};
use alloc::str::FromStr;
use core::fmt;
use proc_macro2::{Delimiter, Group, Span, TokenStream as TS2, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::Ident;

/// HTTP Method enum
/// only Get method is somewhat supported at the moment
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Method {
    Head,
    #[default]
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Connect,
    Options,
    Trace,
}

impl ToTokens for Method {
    fn to_tokens(&self, tokens: &mut TS2) {
        tokens.append(<Method as Into<TokenTree>>::into(*self))
    }
}

impl From<Method> for TokenTree {
    fn from(m: Method) -> Self {
        let var = Ident::new(m.as_str(), Span::call_site());
        Group::new(Delimiter::None, quote::quote! {Method::#var}).into()
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.as_str().chars().next().unwrap(),
            &self.as_str()[1..].to_lowercase(),
        )
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
            b"HEAD" => Ok(Self::Head),
            b"GET" => Ok(Self::Get),
            b"POST" => Ok(Self::Post),
            b"PUT" => Ok(Self::Put),
            b"PATCH" => Ok(Self::Patch),
            b"DELETE" => Ok(Self::Delete),
            b"CONNECT" => Ok(Self::Connect),
            b"OPTIONS" => Ok(Self::Options),
            b"TRACE" => Ok(Self::Trace),
            _ => Err(Self::Error::ClientError(ClientError::BadRequest)),
        }
    }
}

impl TryFrom<&str> for Method {
    type Error = PheasantError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_uppercase().as_str() {
            "HEAD" => Ok(Self::Head),
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "PATCH" => Ok(Self::Patch),
            "DELETE" => Ok(Self::Delete),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            _ => Err(Self::Error::ClientError(ClientError::BadRequest)),
        }
    }
}

impl FromStr for Method {
    type Err = PheasantError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "HEAD" => Ok(Self::Head),
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "PATCH" => Ok(Self::Patch),
            "DELETE" => Ok(Self::Delete),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            _ => Err(Self::Err::ClientError(ClientError::BadRequest)),
        }
    }
}

impl<I> TryFrom<I> for Method
where
    I: Iterator<Item = u8>,
{
    type Error = ErrorStatus;

    fn try_from(mut iter: I) -> Result<Self, Self::Error> {
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
