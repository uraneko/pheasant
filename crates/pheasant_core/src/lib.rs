#![no_std]
#![forbid(clippy::unwrap_used, clippy::expect_used)]
// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![allow(unused_variables)]

extern crate alloc;
extern crate std;

use alloc::string::{FromUtf8Error, String, ToString};
use alloc::{vec, vec::Vec};
use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};
use core::str::FromStr;
use core::str::Utf8Error;
use hashbrown::HashSet;
use pheasant_uri::Route;

// NOTE indefinitely experimental
// mod monopoly;

pub mod method;
pub mod mime;
pub mod monopoly;
pub mod protocol;
pub mod status;
pub mod wildcardish;

pub use method::Method;
pub use mime::Mime;
pub use monopoly::MonoPoly;
pub use protocol::Protocol;
pub use status::{
    ClientError, ErrorStatus, Informational, Redirection, ResponseStatus, ServerError, Status,
    Successful,
};
pub use wildcardish::WildCardish;

// TODO service macro attr status
// this lets the user pick their status code of choice for their service's response
//
// TODO service macro attr resolve
// this lets the user decide what error code their function
// would fail to if needed
//
// requires that the chosen failure status has a registered failure service with the server
// (server.failure(...))
//

// BUG cross origin POST request ran normally despite only GET method being specified in the
// Access-Control-Allow-Methods header
// this appears to be caused by firefox only sending an Origin header with the Post request
// there was no requesting from firefox's side for any methods or headers, only the client origin

pub struct ByteIterator<I: Iterator<Item = u8>> {
    iter: I,
}

impl<I> ByteIterator<I>
where
    I: Iterator<Item = u8>,
{
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> core::ops::Deref for ByteIterator<I>
where
    I: Iterator<Item = u8>,
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.iter
    }
}

impl<I> core::ops::DerefMut for ByteIterator<I>
where
    I: Iterator<Item = u8>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.iter
    }
}

pub type PheasantResult<T> = Result<T, PheasantError>;

/// crate's main error type
#[derive(Debug)]
pub enum PheasantError {
    ClientError(ClientError),
    ServerError(ServerError),
}

impl Display for PheasantError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl core::error::Error for PheasantError {}

// WARN this is senseless, should be PortIsTaken error variant
impl From<std::io::Error> for PheasantError {
    fn from(_err: std::io::Error) -> Self {
        Self::ClientError(ClientError::BadRequest)
    }
}

impl From<core::num::ParseIntError> for PheasantError {
    fn from(_err: core::num::ParseIntError) -> Self {
        Self::ClientError(ClientError::BadRequest)
    }
}

impl From<Utf8Error> for PheasantError {
    fn from(_err: Utf8Error) -> Self {
        Self::ClientError(ClientError::BadRequest)
    }
}

impl From<FromUtf8Error> for PheasantError {
    fn from(_err: FromUtf8Error) -> Self {
        Self::ClientError(ClientError::BadRequest)
    }
}

impl From<url::ParseError> for PheasantError {
    fn from(_err: url::ParseError) -> Self {
        Self::ClientError(ClientError::BadRequest)
    }
}
