#![no_std]
#![allow(refining_impl_trait)]
extern crate alloc;
use pheasant_core::ErrorStatus;

// pub mod authentication;
// pub mod caching;
// pub mod conditionals;
// pub mod connection_management;
// pub mod content_negotiation;
// pub mod controls;
pub mod cookies;
pub mod cors;
// pub mod deprecated;
// pub mod downloads;
// pub mod experimental;
// pub mod fetch_metadata_request_headers;
// pub mod integrity_digests;
// pub mod integrity_policy;
pub mod message_body_information;
// pub mod non_standard;
pub mod other;
// pub mod preferences;
// pub mod proxies;
// pub mod range_requests;
// pub mod redirects;
pub mod request_context;
// pub mod response_context;
// pub mod security;
// pub mod server_sent_events;
// pub mod transfer_coding;
// pub mod websockets;

pub use cookies::*;
pub use cors::*;
pub use message_body_information::*;
pub use other::*;
pub use request_context::*;

pub mod routing;

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
