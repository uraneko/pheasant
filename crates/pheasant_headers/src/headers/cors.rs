extern crate alloc;
use alloc::{
    borrow::ToOwned,
    format,
    string::{String, ToString},
};
use chrono::TimeDelta;
use core::fmt::{self, Debug, Display, Formatter};
use hashbrown::{HashMap, HashSet};

use crate::{FromHeader, FromHeaders, IntoHeader, IntoHeaders, IterFromHeaders, IterIntoHeaders};
use crate::{Header, Headers};
use pheasant_core::{ClientError, ErrorStatus, Method, WildCardish, err_stt};
use pheasant_uri::Origin;
// TODO Timing-Allow-Origin header

pub mod configs;
pub mod request;
pub mod respond;

pub use configs::CorsConfigs;
pub use request::RequestCors;
pub use respond::RespondCors;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct CorsHeader {
    header: String,
    /// is this header allowed to be exposed to the clinet in cross origin request responses
    expose: bool,
}

impl IntoHeaders<HashSet<String>> for String {
    fn into_headers(self) -> Result<HashSet<String>, ErrorStatus> {
        Ok(self.split(',').map(|s| s.trim().to_owned()).collect())
    }
}

// cookies as a single header
impl From<&HashSet<String>> for Header {
    fn from(h: &HashSet<String>) -> Header {
        let mut s = h
            .into_iter()
            .fold("".to_owned(), |acc, h| acc + h.as_str() + ", ");
        s.pop();
        s.pop();

        Header::Field(s)
    }
}
