#![no_std]
#![forbid(clippy::unwrap_used, clippy::expect_used)]
extern crate alloc;

pub mod headers;
pub mod maybe_glob;
pub mod message;
pub mod method;
pub mod protocol;
pub mod request;
pub mod respond;
pub mod status;

pub use headers::{Header, contains_header, header_value};
pub use maybe_glob::MaybeGlob;
pub use method::Method;
pub use protocol::Protocol;
pub use request::Request;
pub use respond::Respond;
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

pub trait ConversionError {}

impl<C: ConversionError> From<C> for ErrorStatus {
    fn from(_err: C) -> Self {
        err_stt!(400)
    }
}

pub fn sidestep_whitespace(buf: &[u8], mut idx: usize) -> usize {
    while buf[idx] == 32 {
        idx += 1;
    }

    idx
}
