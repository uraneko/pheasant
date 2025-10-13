#![no_std]
#![allow(refining_impl_trait)]
extern crate alloc;
use pheasant_core::ErrorStatus;

pub mod headers;
pub use headers::*;

// conversions from header types
/// converts a header type into Self
pub trait FromHeader<H> {
    fn from_header(h: H) -> Self;
}

/// converts many headers into Self
pub trait FromHeaders<H> {
    fn from_headers(h: H) -> Self;
}

/// converts an iterator of headers into self
pub trait IterFromHeaders<H> {
    fn iter_from_headers(h: H) -> impl IntoIterator<Item = Self>;
}

// conversions to header types
/// converts Self into a header type
pub trait IntoHeader<H> {
    fn into_header(self) -> Result<H, ErrorStatus>;
}

/// converts Self into many headers
pub trait IntoHeaders<H> {
    fn into_headers(self) -> Result<H, ErrorStatus>;
}

/// converts an iterator of self into a groups of header
pub trait IterIntoHeaders<H> {
    fn iter_into_headers(i: impl IntoIterator<Item = Self>) -> Result<H, ErrorStatus>;
}
