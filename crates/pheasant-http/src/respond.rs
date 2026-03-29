use crate::message::{
    Token,
    http11::{Error, Lex, build_headers, content_length},
};
use crate::{Header, Method, Protocol};
use crate::{Status, status};
use alloc::string::String;
use alloc::vec::Vec;
use embedded_io::{Read, Write};
use pheasant_uri::{Path, Query};

#[derive(Debug)]
pub struct Respond<H: core::fmt::Debug + Clone> {
    proto: Protocol,
    status: Status,
    headers: H,
    body: Vec<u8>,
}

impl<'a> Lex<'a> {
    pub fn respond(&mut self) -> Result<Respond<Vec<Header>>, Error> {
        let proto = self.resp_proto()?;
        let status = self.status()?;
        let headers = self.headers()?;
        let len = content_length(&headers);
        let len = match len {
            Err(Error::ContentLengthNotFound) => self.len() - self.cursor,
            Ok(len) => len,
            Err(err) => return Err(err),
        };
        let body = match self.body(len)? {
            Some(Token::Body(body)) => body,
            Some(_) => return Err(Error::UndesirableToken),
            None => Vec::new(),
        };
        let headers = build_headers(headers)?;

        Ok(Respond {
            proto,
            status,
            headers,
            body,
        })
    }
}

impl Respond<Vec<Header>> {
    pub fn proto(&self) -> Protocol {
        self.proto
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn headers(&self) -> &[Header] {
        &self.headers
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }
}

// NOTE this function would have been easier on the eyes had it used self.iter.position and field.len
// fn find_subslice(slice: &[u8], subslice: &[u8]) -> Option<usize> {
//     // error the only way a single header field is equal in len to the entire headers buffer is
//     // if something is wrong
//     if subslice.len() >= slice.len() {
//         return None;
//     }
//     let mut idx = 0;
//     let mut cursor = 0;
//     while cursor < slice.len() {
//         let byte = slice[cursor];
//         if subslice[idx] == byte {
//             if idx == subslice.len() - 1 {
//                 break;
//             }
//             idx += 1;
//         } else {
//             idx = 0;
//         }
//         cursor += 1;
//     }
//
//     if cursor == slice.len() {
//         return None;
//     }
//
//     // error there should have been a colon right after the header field
//     if slice[cursor + 1] != b':' {
//         return None;
//     }
//
//     Some(cursor)
// }

// fn map_value(slice: &[u8], mut idx: usize) -> Option<&[u8]> {
//     let sub = &slice[idx + 2..];
//     idx = 0;
//     while idx < sub.len() {
//         match sub[idx] {
//             b'\r' | b'\n' => break,
//             _ => continue,
//         }
//     }
//
//     // error there should have been a line break of some sort after the header value
//     if idx == sub.len() - 1 {
//         return None;
//     }
//
//     Some(sub[..idx].trim_ascii())
// }

// impl ReadHeaders for Vec<u8> {
//     fn find(&self, field: &[u8]) -> Option<&[u8]> {
//         let idx = find_subslice(&self, field)?;
//
//         map_value(&self, idx)
//     }
//
//     fn find_all(&self, field: &[u8]) -> Option<&[&[u8]]> {
//         let Some(mut value) = self.find(field) else {
//             return None;
//         };
//         while let Some(find) = find_next(&self, value) {
//             value = find;
//         }
//         None
//     }
// }

impl Respond<Vec<u8>> {
    pub fn new(proto: Protocol, status: Status) -> Self {
        Self {
            proto,
            status,
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn proto(&mut self, proto: Protocol) -> &mut Self {
        self.proto = proto;

        self
    }

    pub fn proto_cpy(&self) -> Protocol {
        self.proto
    }

    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = status;

        self
    }

    pub fn status_cpy(&self) -> Status {
        self.status
    }

    pub fn headers_mut(&mut self) -> &mut Vec<u8> {
        &mut self.headers
    }

    pub fn headers_ref(&self) -> &[u8] {
        &self.headers
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }

    pub fn body_ref(&self) -> &[u8] {
        &self.body
    }

    /// returns an iterator of the response bytes correctly formatted for sending
    ///
    /// includes the response body data
    ///
    /// this clears the contents of the headers and body buffers
    pub fn stream_bytes(&self) -> impl IntoIterator<Item = u8> {
        let stream = self
            .proto
            .as_bytes()
            .into_iter()
            .chain(Some(&32))
            .chain(self.status.as_bytes())
            .chain(Some(&10))
            .chain(&self.headers)
            .chain(Some(&10))
            .chain(self.body.as_slice())
            .copied();

        // TODO needs read to end
        // let _n = self.headers.read(hbuf).unwrap();
        // let stream = stream.chain(hbuf.into_iter().map(|b| *b)).chain(Some(10));
        // let _n = self.body.read(bbuf).unwrap();
        // let stream = stream.chain(bbuf.into_iter().map(|b| *b));

        stream
    }

    /// returns an iterator of the response bytes correctly formatted for sending
    ///
    /// assumes response has no body data or a body is inadequate for the request method, use when
    /// body is surely empty or the request method is head or connect
    ///
    /// this clears the contents of the headers buffer
    // pub fn stream_bytes_nodata(&self) -> impl IntoIterator<Item = u8> {
    //     let stream = self
    //         .proto
    //         .as_bytes()
    //         .into_iter()
    //         .chain(Some(&32))
    //         .chain(self.status.as_bytes())
    //         .chain(Some(&10))
    //         .chain(&self.headers)
    //         .chain(Some(&10))
    //         .map(|b| *b);
    //
    //     // TODO needs read to end
    //     // let _n = self.headers.read(buf).unwrap();
    //     // let stream = stream.chain(buf.into_iter().map(|b| *b)).chain(Some(10));
    //
    //     stream
    // }

    // pub fn read_headers(&mut self, buf: &mut [u8]) -> Result<usize, H::Error> {
    //     self.headers.read(buf)
    // }
    //
    // pub fn read_body(&mut self, buf: &mut [u8]) -> Result<usize, B::Error> {
    //     self.body.read(buf)
    // }

    // resets proto and status to defaults
    // and clears headers and body
    pub fn clear(&mut self) {
        self.proto = Protocol::Http11;
        self.status = status!(200);
        self.headers.clear();
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        !self.body.is_empty()
    }
}

impl From<Respond<Vec<Header>>> for Respond<Vec<u8>> {
    fn from(resp: Respond<Vec<Header>>) -> Self {
        let Respond {
            proto,
            status,
            headers,
            body,
        } = resp;
        let headers = crate::headers::headers_into_bytes(headers);

        Self {
            proto,
            status,
            headers,
            body,
        }
    }
}
