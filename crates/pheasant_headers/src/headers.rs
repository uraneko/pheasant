extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use hashbrown::{HashMap, HashSet};
extern crate std;
use std::io::Write;

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

// pub mod groups {
//     use super::{cookies, cors, message_body_information, other, request_context};
//
//     pub use cookies::Cookie;
//     pub use cors::{RequestCors, ResourceCors, ResponseCors};
//     pub use message_body_information::{
//         ContentEncoding, ContentEncodingBits, Encoding, SetContentLength,
//     };
// }

pub use cookies::*;
pub use cors::*;
pub use message_body_information::*;
pub use other::*;
pub use request_context::*;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Headers {
    headers: HashMap<String, Header>,
    _slice: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Header {
    Field(String),
    Set(HashSet<String>),
}

impl Header {
    pub fn len(&self) -> usize {
        match self {
            Self::Field(f) => f.len(),
            Self::Set(s) => s.iter().map(|v| v.len()).sum(),
        }
    }
}

impl Headers {
    /// sets a new header single field value
    pub fn header(&mut self, h: impl ToString, f: impl ToString) {
        self.headers
            .insert(h.to_string(), Header::Field(f.to_string()));
    }

    /// inserts a new header with a set value filled with the passed field value
    pub fn headers(&mut self, h: impl ToString, f: impl ToString) {
        self.headers
            .insert(h.to_string(), Header::Set(HashSet::from([f.to_string()])));
    }

    /// inserts a new header with a set value from the passed iterator
    pub fn headers_from_iter(&mut self, h: impl ToString, f: impl IntoIterator<Item = String>) {
        self.headers
            .insert(h.to_string(), Header::Set(HashSet::from_iter(f)));
    }

    /// insert field value into existing set
    ///
    /// makes a new one if it doesnt exist
    pub fn insert(&mut self, h: impl ToString + AsRef<str>, f: impl ToString) {
        let Some(Header::Set(s)) = self.headers.get_mut(h.as_ref()) else {
            self.headers(h, f);

            return;
        };

        s.insert(f.to_string());
    }

    /// checks if a headers is already set
    pub fn contains(&self, key: &str) -> bool {
        self.headers.contains_key(key)
    }

    pub fn is_field(&self, key: &str) -> bool {
        let Some(f) = self.headers.get(key) else {
            return false;
        };

        f.is_field()
    }

    /// removes field header value if it exists
    pub fn remove(&mut self, key: &str) -> Option<String> {
        if !self.is_field(key) {
            return None;
        }

        let Some(Header::Field(f)) = self.headers.remove(key) else {
            return None;
        };

        Some(f)
    }

    /// removes a set header value if it exists
    pub fn extract(&mut self, key: &str) -> Option<HashSet<String>> {
        if self.is_field(key) {
            return None;
        }

        let Some(Header::Set(s)) = self.headers.remove(key) else {
            return None;
        };

        Some(s)
    }

    /// removes specified headers and returns them in a Header with field _slice = true
    ///
    /// which is a Headers instance with a different name to indicate that it is a partial instance
    /// that was sliced off another
    pub fn slice_off<'a>(&mut self, keys: impl IntoIterator<Item = &'a str>) -> Headers {
        keys.into_iter()
            .map(|k| {
                if self.is_field(k) {
                    self.remove(k).map(|h| h.into())
                } else {
                    self.extract(k).map(|h| h.into())
                }
                .map(|h| (k.to_owned(), h))
            })
            .flatten()
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }

    pub fn write_to(self, mut buf: &mut [u8]) -> Result<(), std::io::Error> {
        extern crate std;
        use std::io::Write;
        for (k, v) in self.headers.into_iter() {
            buf.write(k.as_bytes())?;
            buf.write(b":")?;
            match v {
                Header::Field(f) => {
                    buf.write(f.as_bytes())?;
                    buf.write(&[10])?;
                    buf.flush()?;
                }
                Header::Set(s) => {
                    // This means im writing repeating headers as a single header
                    buf.write(unify_header_fields(s).as_bytes())?;
                    buf.write(&[10])?;
                    buf.flush()?;
                }
            }
        }

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Header)> {
        self.headers.iter()
    }
}

// for writing all field values of the same header key into the same header
fn unify_header_fields(i: impl IntoIterator<Item = String>) -> String {
    let mut f = i
        .into_iter()
        .fold("".to_owned(), |acc, v| acc + v.as_str() + ", ");
    f.pop();
    f.pop();

    f
}

// for writing many individual fields for the same header key
fn separate_header_fields(
    k: &str,
    i: impl IntoIterator<Item = String>,
    mut buf: &mut [u8],
) -> Result<(), std::io::Error> {
    for v in i.into_iter() {
        buf.write(k.as_bytes())?;
        buf.write(b":")?;
        buf.write(v.as_bytes())?;
        buf.write(&[10])?;
        buf.flush()?;
    }

    Ok(())
}

impl From<String> for Header {
    fn from(s: String) -> Header {
        Header::Field(s)
    }
}

impl From<HashSet<String>> for Header {
    fn from(s: HashSet<String>) -> Header {
        Header::Set(s)
    }
}

impl FromIterator<(String, Header)> for Headers {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (String, Header)>,
    {
        Self {
            _slice: true,
            headers: iter.into_iter().collect(),
        }
    }
}

impl Header {
    pub fn is_field(&self) -> bool {
        let Self::Field(_) = self else {
            return false;
        };

        true
    }

    pub fn is_set(&self) -> bool {
        let Self::Set(_) = self else {
            return false;
        };

        true
    }
}
