extern crate alloc;
use alloc::{borrow::ToOwned, string::String};

use crate::Headers;
use hashbrown::HashSet;
use pheasant_core::{ErrorStatus, Method, WildCardish, err_stt};
use pheasant_uri::Origin;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestCors {
    /// the cors req origin, client MUST provide this header
    origin: WildCardish<Origin>,
    /// client MAY send this
    headers: Option<HashSet<String>>,
    /// client MUST send this
    method: Method,
}

impl RequestCors {
    pub fn from_headers(mut h: Headers) -> Result<Self, ErrorStatus> {
        let (Some(Ok(origin)), Some(Ok(method))) = (
            h.remove("Origin").map(|o| o.parse()),
            h.remove("Access-Control-Request-Method").map(|m| m.parse()),
        ) else {
            // NOTE could also use 422 maybe
            return err_stt!(?BadRequest);
        };

        let headers = h
            .remove("Access-Control-Request-Headers")
            .map(parse_headers)
            .transpose()?;

        Ok(RequestCors {
            origin,
            method,
            headers,
        })
    }
}

fn parse_headers(s: String) -> Result<HashSet<String>, ErrorStatus> {
    Ok(s.split(',').map(|s| s.trim().to_owned()).collect())
}
