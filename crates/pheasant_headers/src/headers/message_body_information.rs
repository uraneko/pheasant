use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::fmt::Debug;
use core::str::FromStr;
use mime::Mime;

use pheasant_core::{ClientError, ErrorStatus, ServerError, WildCardish};
use pheasant_uri::Origin;

use crate::{FromHeader, HttpResult, ToHeader, ToHeaders};

pub struct SetContentLength<'a>(&'a [u8]);

impl<'a> SetContentLength<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self(slice)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> ToHeader for SetContentLength<'a> {
    fn to_header(&self) -> String {
        self.len().to_string()
    }
}

pub struct ContentLength(usize);

impl FromHeader for ContentLength {
    fn from_header(header: String) -> HttpResult<Self> {
        match header.parse::<usize>().unwrap() {
            size if size < 8192 => Ok(Self(size)),
            // TODO validation should be done later since i have no way of knowing
            // server content size limits at this point
            _ => Err(ErrorStatus::Client(ClientError::ContentTooLarge)),
        }
    }
}

pub struct ContentType(Mime);

impl ContentType {
    pub fn new(mime: &str) -> Self {
        Self(mime.parse().unwrap())
    }

    pub fn mime(&self) -> &Mime {
        &self.0
    }
}

impl ToHeader for ContentType {
    fn to_header(&self) -> &str {
        self.mime().essence_str()
    }
}

impl FromHeader for ContentType {
    fn from_header(header: String) -> HttpResult<Self> {
        match header
            .parse::<Mime>()
            .map_err(|_| ErrorStatus::Server(ServerError::NotImplemented))
        {
            Ok(mime) => Ok(Self(mime)),
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Encoding {
    Deflate,
    Gzip,
    Zlib,
}

impl FromStr for Encoding {
    type Err = ErrorStatus;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "deflate" => Ok(Self::Deflate),
            "gzip" => Ok(Self::Gzip),
            "zlib" => Ok(Self::Zlib),
            "br" | "zstd" | "dcb" | "dcz" => Err(ErrorStatus::Server(ServerError::NotImplemented)),
            // may be a bad/non-existent algorithm name
            // or
            // an algorithm that this lib doesnt know about
            _ => Err(ErrorStatus::Client(ClientError::UnprocessableContent)),
        }
    }
}

impl Encoding {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Deflate => 1,
            Self::Gzip => 2,
            Self::Zlib => 4,
        }
    }

    pub fn encode(&self, slice: &[u8]) -> Vec<u8> {
        match self {
            Self::Deflate => deflate::deflate_bytes(slice),
            Self::Gzip => deflate::deflate_bytes_gzip(slice),
            Self::Zlib => deflate::deflate_bytes_zlib(slice),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContentEncodingBits(pub u8);

impl ContentEncodingBits {
    pub fn encoding_list(self) -> &'static str {
        match self.0 {
            0 => "",
            1 => "deflate",
            2 => "gzip",
            3 => "deflate, gzip",
            4 => "zlib",
            5 => "deflate, zlib",
            6 => "gzip, zlib",
            7 => "deflate, gzip, zlib",
            _ => unimplemented!("reached unimplemented encodings"),
        }
    }
}

impl ToHeader for ContentEncodingBits {
    fn to_header(&self) -> &str {
        self.encoding_list()
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

impl FromHeader for ContentEncoding {
    fn from_header(header: String) -> HttpResult<Self> {
        Ok(Self {
            encodings: header
                .split(',')
                .map(|algo| algo.parse::<Encoding>().unwrap())
                .collect(),
        })
    }
}

// not gonna use
//
// impl Resposne {
//     fn encode_body(&mut self, encoder: Encoding) -> ContentEncoding {
//         ContentEncoding::new(
//             || {
//                 *self.body = encoder.encode(&self.body);
//
//                 &mut self.body
//             },
//             encoder.to_u8(),
//         )
//     }
// }
//
// pub struct ContentEncoding<I> {
//     inner: I,
//     encodings: u8,
// }
//
// impl<I> ContentEncoding<I> {
//     fn new(inner: I, encodings: u8) -> Self {
//         Self { inner, encodings }
//     }
//
//     fn encode(self, encoder: Encoding) -> Self {
//         ContentEncoding::new(
//             || {
//                 let bytes: &mut Vec<u8> = self.inner();
//                 *bytes = encoder.encode(&bytes);
//
//                 bytes
//             },
//             self.encodings | encoder.to_u8(),
//         )
//     }
//
//     fn content_encoding(self) {
//         self.inner()
//     }
// }
//
// impl ToHeader for ContentEncoding {
//     type Output = [&str; 2];
//
//     fn to_header(&self, h: &str) -> Self::Output {}
// }
//
//

// TODO
pub struct ContentLanguage {}

// TODO
pub struct ContentLocation {}
