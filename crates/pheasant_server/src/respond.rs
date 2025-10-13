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
use pheasant_headers::{Cookie, Headers, RespondCors};

pub mod builder;
pub mod http11;

pub use builder::Builder;
pub use http11::ScrutinizeCors;

pub struct Respond {
    proto: Protocol,
    status: Status,
    body: Option<Vec<u8>>,
    headers: Headers,
    // Builder should work out all headers when generating a Respond
    // i,e,. Builder has these fields but respond itself has no need for them
    // cookies: Option<HashSet<Cookie>>,
    // cors: Option<ResponseCors<'a>>,
}

impl Respond {
    pub fn builder<'a>(status: Status, proto: Protocol, cross_origin: bool) -> Builder<'a> {
        Builder::new(status, proto, cross_origin)
    }
}

impl Respond {
    pub fn proto(&self) -> Protocol {
        self.proto
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn error(err: ErrorStatus, proto: Protocol) -> Self {
        Self {
            body: None,
            headers: Headers::default(),
            status: err.into(),
            proto,
        }
    }
}

impl core::fmt::Debug for Respond {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Respond {{\n   {} {} {}\n   headers: {:#?}\n   body: {:?}\n}}",
            self.proto,
            self.status.code(),
            self.status.text(),
            self.headers,
            self.body
                .as_ref()
                .map(|b| if b.len() > 19 {
                    format!("{:?}...", &b[..20])
                } else {
                    format!("{:?}", &b)
                })
                .unwrap_or_default()
        )
    }
}
