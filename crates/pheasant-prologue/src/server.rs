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

#[derive(Debug, Clone)]
pub struct Request {
    pub(crate) method: Method,
    pub(crate) path: Path,
    pub(crate) query: Option<Query>,
    pub(crate) proto: Protocol,
    pub(crate) headers: Vec<Header>,
    pub(crate) body: Option<Vec<u8>>,
}

impl<'a> Lex<'a> {
    pub fn request(&mut self) -> Result<Request, Error> {
        let method = self.method()?;
        let (path, query) = self.url()?.disassemble();
        let (proto, _) = self.req_proto()?;
        let headers = self.headers()?;
        let len = content_length(&headers);
        let len = match len {
            // we take from cursor to buffer end
            Err(Error::ContentLengthNotFound) => self.len() - self.cursor,
            Ok(len) => len,
            Err(err) => return Err(err),
        };
        let body = match self.body(len)? {
            Some(Token::Body(body)) => Some(body),
            Some(_) => return Err(Error::UndesirableToken),

            None => None,
        };
        let headers = build_headers(headers)?;

        Ok(Request {
            method,
            path,
            query,
            proto,
            headers,
            body,
        })
    }
}

impl Request {
    pub fn method(&self) -> Method {
        self.method
    }

    pub fn proto(&self) -> Protocol {
        self.proto
    }

    pub fn path(&self) -> &[String] {
        &self.path.segments()
    }

    pub fn path_str(&self) -> String {
        self.path.serialized()
    }

    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }

    pub fn query_mut(&mut self) -> Option<&mut Query> {
        self.query.as_mut()
    }

    pub fn headers(&self) -> &[Header] {
        &self.headers
    }

    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_ref().map(|b| b.as_slice())
    }
}

impl Request {
    /// makes a header value all in lowercase if it exists in request headers
    /// useful for using any service that only works when a request header value matches with one that you provided to the service
    /// and you're not sure about the request header value's case (upper/lower)
    ///
    /// # example
    /// client sends this header: 'access-control-request-headers: Range, Content-type'
    /// the server has this cors configuration: 'access-control-allow-headers: range, content-type'
    /// the headers are allowed, but the Cors service would not understand that
    /// since range != Range && Content-type != content-type
    /// so we lowercase the request header value first
    ///
    /// ```
    /// use pheasant_prologue::message::http11::{Error, Lex};
    ///
    /// let mut req = Lex::new(b"GET / HTTP/1.1\naccess-control-request-method: GET\naccess-control-request-header: ranges\norigin: localhost\n\n").request()?;
    /// req.lowercase_header_value(b"access-control-request-headers");
    ///
    /// Ok::<(), Error>(())
    /// ```
    pub fn lowercase_header_value(&mut self, field: &[u8]) {
        let Some(value) = self
            .headers
            .iter_mut()
            .find_map(|h| (h.field_ref() == field).then(|| h.value_mut()))
        else {
            return;
        };

        value.make_ascii_lowercase();
    }

    /// same as the lowercase_header_value method but does many headers' values at once
    pub fn lowercase_header_values(&mut self, fields: &[&[u8]]) {
        self.headers
            .iter_mut()
            .filter(|h| fields.contains(&h.field_ref()))
            .for_each(|h| h.value_mut().make_ascii_lowercase())
    }
}

#[derive(Debug)]
pub struct Respond {
    pub(crate) proto: Protocol,
    pub(crate) status: Status,
    pub(crate) headers: Vec<u8>,
    pub(crate) body: Vec<u8>,
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

impl Respond {
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
