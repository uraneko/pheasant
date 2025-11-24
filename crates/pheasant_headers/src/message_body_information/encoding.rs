use alloc::borrow::ToOwned;
use alloc::{string::String, vec::Vec};
use core::fmt::Debug;
use core::str::FromStr;

use pheasant_core::{ClientError, ErrorStatus, ServerError};

pub const GZIP: Encoding = Encoding::Gzip;
pub const DEFLATE: Encoding = Encoding::Deflate;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Encoding {
    Deflate,
    Gzip,
    // Zlib,
}

impl FromStr for Encoding {
    type Err = ErrorStatus;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "deflate" => Ok(Self::Deflate),
            "gzip" => Ok(Self::Gzip),
            // "zlib" => Ok(Self::Zlib),
            "br" | "zstd" | "dcb" | "dcz" => Err(ErrorStatus::Server(ServerError::NotImplemented)),
            // may be a bad/non-existent algorithm name
            // or
            // an algorithm that this lib doesnt know about
            _ => Err(ErrorStatus::Client(ClientError::UnprocessableContent)),
        }
    }
}

impl Encoding {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Deflate => "deflate",
            Self::Gzip => "gzip",
        }
    }

    pub fn encode(&self, slice: &[u8]) -> Vec<u8> {
        match self {
            Self::Deflate => deflate::deflate_bytes(slice),
            Self::Gzip => deflate::deflate_bytes_gzip(slice),
            // Self::Zlib => deflate::deflate_bytes_zlib(slice),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ContentEncoding {
    encodings: Vec<Encoding>,
}

// impl FromIterator<Encoding> for ContentEncoding {
//     fn from_iter<T>(iter: T) -> Self
//     where
//         T: IntoIterator<Item = Encoding>,
//     {
//         Self {
//             encodings: iter.collect(),
//         }
//     }
// }

impl ContentEncoding {
    pub fn new(enc: Encoding) -> Self {
        Self {
            encodings: Vec::from([enc]),
        }
    }

    pub fn encoding(&mut self, enc: Encoding) -> &mut Self {
        self.encodings.push(enc);

        self
    }

    /// applies encodings to the body
    pub fn encode(self, body: Vec<u8>) -> Vec<u8> {
        self.encodings
            .into_iter()
            .fold(body, |acc, enc| enc.encode(&acc))
    }

    /// this is generating a new content encoding header value from the read request bytes
    pub fn from_request_header(s: String) -> Result<ContentEncoding, ErrorStatus> {
        Ok(ContentEncoding {
            encodings: s
                .split(',')
                .map(|algo| algo.parse::<Encoding>().unwrap())
                .collect(),
        })
    }

    pub fn to_header(&self) -> String {
        let mut h = self
            .encodings
            .iter()
            .fold("".to_owned(), |acc, enc| acc + enc.as_str() + ", ");
        h.pop();
        h.pop();

        h
    }
}
