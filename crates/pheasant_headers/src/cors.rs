//! How do Cors work
//!
//! - user sets cors permissions on the servlet
//! - request parses request cors if they exist
//! - respond sets respond cors if servlet and request have them
extern crate alloc;
use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
};
use core::fmt::Debug;
use core::str::FromStr;
use hashbrown::HashSet;

use crate::{FromHeaders, IntoHeader, IntoHeaders, IterFromHeaders, IterIntoHeaders};
use pheasant_core::{ErrorStatus, Header};
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

// impl IntoHeaders<HashSet<String>> for String {
//     fn into_headers(self) -> Result<HashSet<String>, ErrorStatus> {
//         Ok(self.split(',').map(|s| s.trim().to_owned()).collect())
//     }
// }

// cookies as a single header
// impl From<&HashSet<String>> for Header {
//     fn from(h: &HashSet<String>) -> Header {
//         let mut s = h
//             .into_iter()
//             .fold("".to_owned(), |acc, h| acc + h.as_str() + ", ");
//         s.pop();
//
//         Header::Field(s)
//     }
// }

fn deserialize_many_into_one<T: FromStr + Eq + core::hash::Hash>(s: &str) -> HashSet<T> {
    s.split(',').filter_map(|s| s.parse::<T>().ok()).collect()
}

fn deserialize_set_into_set<T>(set: HashSet<String>) -> HashSet<T> {
    let iter: Result<_, ErrorStatus> = s
        .into_iter()
        .map(move |mut header| {
            if let Some(kv) = take_key_val(&mut header) {
                let [ref k, ref v] = kv?;

                Cookie::new(k, v).fill_out(&mut header)
            } else {
                err_stt!(?NotImplemented)
            }
        })
        .collect();
    iter
}

fn serialize_set<T: ToString>(set: HashSet<T>) -> String {
    let mut s = set
        .into_iter()
        .fold("".to_string(), |acc, t| acc + &t.to_string() + ",");
    s.pop();

    s
}
