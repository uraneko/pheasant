//! this crate defines some primitive types apis
//!
//! ### APIs
//! - Method
//! - Protocol
//! - Status
//! - Wildcardish

#![no_std]
#![forbid(clippy::unwrap_used, clippy::expect_used)]

extern crate alloc;
extern crate std;

use alloc::string::FromUtf8Error;
use core::fmt::{self, Debug, Display, Formatter};
use core::str::Utf8Error;

pub mod client;
pub mod headers;
pub mod maybe_glob;
pub mod message;
pub mod method;
pub mod mime;
pub mod protocol;
pub mod server;
pub mod status;

pub use headers::{Header, contains_header, header_value};
pub use method::Method;
pub use mime::Mime;
// pub use monopoly::MonoPoly;
pub use maybe_glob::MaybeGlob;
pub use protocol::Protocol;
pub use status::{
    ClientError, ErrorStatus, Informational, Redirection, ServerError, Status, Successful,
};

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

#[macro_export]
macro_rules! repeat_tfs {
    ($t: ty) => {
        impl<'a> TryFrom<&'a str> for $t {
            type Error = <Self as FromStr>::Err;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                s.parse()
            }
        }
    };
}

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

pub fn sidestep_whitespace(buf: &[u8], mut idx: usize) -> usize {
    while buf[idx] == 32 {
        idx += 1;
    }

    idx
}
