//! this crate defines the respond api
//!
//! ### APIs
//! - parse (from respond into bytes)
//! - builder
//! - headers (write)

use crate::Request;
use hashbrown::{HashMap, HashSet};
use mime::Mime;
use pheasant_core::{ErrorStatus, Protocol, Status, StatusLiterals, Successful};
use pheasant_middleware::{Cookie, Headers, RespondCors};

pub mod builder;
pub mod http11;

pub use builder::Builder;

pub struct Respond<'a> {
    proto: Protocol,
    status: Status,
    headers: Headers,
    body: &'a mut Vec<u8>,
}

impl<'a> Respond<'a> {
    pub fn proto(&self) -> Protocol {
        self.proto
    }

    // WARN FIXME code smell
    // Respond cant implement methods that mutate itself
    // those should be limited to Builder
    pub fn status(&mut self, s: Status) {
        self.status = s;
    }

    pub fn error(err: ErrorStatus, proto: Protocol, body: &'a mut Vec<u8>) -> Self {
        Self {
            body,
            headers: Headers::default(),
            status: err.into(),
            proto,
        }
    }

    pub fn is_cross_origin(&self) -> bool {
        self.headers.contains("Access-Control-Allow-Origin")
            && self.headers.contains("Access-Control-Allow-Methods")
    }
}

impl core::fmt::Debug for Respond<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Respond {{\n   {} {} {}\n   headers: {:#?}\n   body: {:?}\n}}",
            self.proto,
            self.status.code(),
            self.status.text(),
            self.headers,
            if self.body.len() > 19 {
                format!("{:?}...", &self.body[..20])
            } else {
                format!("{:?}", &self.body)
            }
        )
    }
}
