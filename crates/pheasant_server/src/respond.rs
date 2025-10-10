//! this crate defines the respond api
//!
//! ### APIs
//! - parse (from respond into bytes)
//! - builder
//! - headers (write)

use crate::Request;
use hashbrown::{HashMap, HashSet};
use mime::Mime;
use pheasant_core::{ErrorStatus, Protocol, Status, Successful};
use pheasant_headers::{Cookie, Headers, RespondCors};

pub mod builder;
pub mod http11;

use builder::Builder;

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
    pub fn builder<'a>(status: Status, proto: Protocol) -> Builder<'a> {
        Builder::new(status, proto)
    }
}

impl Respond {
    pub fn proto(&self) -> Protocol {
        self.proto
    }

    pub fn status(&self) -> Status {
        self.status
    }
}
