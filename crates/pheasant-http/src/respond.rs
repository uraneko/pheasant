use crate::message::{
    Token,
    http11::{Error, Lex, build_headers, content_length},
};
use crate::{Header, Headers, Protocol};
use crate::{Status, status};
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Respond {
    proto: Protocol,
    status: Status,
    headers: Headers,
    body: Vec<u8>,
}

impl Respond {
    pub fn new(proto: Protocol, status: Status) -> Self {
        Self {
            proto,
            status,
            headers: Headers::default(),
            body: Vec::new(),
        }
    }
}
impl<'a> Lex<'a> {
    pub fn respond(&mut self) -> Result<Respond, Error> {
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
        let headers = build_headers(headers)?.into();

        Ok(Respond {
            proto,
            status,
            headers,
            body,
        })
    }
}

#[derive(Debug)]
pub struct Client(Respond);

impl core::ops::Deref for Client {
    type Target = Respond;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Client {
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

    pub fn take_body(&mut self) -> Vec<u8> {
        core::mem::take(&mut self.body)
    }

    pub fn take_headers(&mut self) -> Headers {
        core::mem::take(&mut self.headers)
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }
}

#[derive(Debug)]
pub struct ClientMut<'a>(&'a mut Respond);

impl<'a> core::ops::Deref for ClientMut<'a> {
    type Target = &'a mut Respond;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for ClientMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> ClientMut<'a> {
    pub fn take_body(&mut self) -> Vec<u8> {
        core::mem::take(&mut self.body)
    }

    pub fn take_headers(&mut self) -> Headers {
        core::mem::take(&mut self.headers)
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }
}

#[derive(Debug)]
pub struct ClientRef<'a>(&'a Respond);

impl core::ops::Deref for ClientRef<'_> {
    type Target = Respond;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ClientRef<'a> {
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

#[derive(Debug)]
pub struct Server(Respond);

impl core::ops::Deref for Server {
    type Target = Respond;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Server {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Server {
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

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn headers_ref(&self) -> &[Header] {
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
            .copied()
            .chain(self.headers.stream_bytes())
            .chain(Some(10))
            .chain(self.body.as_slice().into_iter().map(|b| *b));

        // TODO needs read to end
        // let _n = self.headers.read(hbuf).unwrap();
        // let stream = stream.chain(hbuf.into_iter().map(|b| *b)).chain(Some(10));
        // let _n = self.body.read(bbuf).unwrap();
        // let stream = stream.chain(bbuf.into_iter().map(|b| *b));

        stream
    }

    pub fn has_body(&self) -> bool {
        !self.body.is_empty()
    }

    // resets proto and status to defaults
    // and clears headers and body
    pub fn clear(&mut self) {
        self.proto = Protocol::Http11;
        self.status = status!(200);
        self.headers.clear();
        self.body.clear();
    }

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    /// removes all headers expect for those in excluded
    pub fn clear_headers_exclude(&mut self, excluded: &[&[u8]]) {
        self.headers.retain(|h| excluded.contains(&h.field_ref()))
    }

    /// removes only the headers that are in included
    pub fn clear_headers_include(&mut self, included: &[&[u8]]) {
        self.headers.retain(|h| !included.contains(&h.field_ref()))
    }

    /// removes all headers
    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }
}

#[derive(Debug)]
pub struct ServerMut<'a>(&'a mut Respond);

impl<'a> core::ops::Deref for ServerMut<'a> {
    type Target = &'a mut Respond;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for ServerMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> ServerMut<'a> {
    pub fn proto(&mut self, proto: Protocol) -> &mut Self {
        self.proto = proto;

        self
    }

    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = status;

        self
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }

    // resets proto and status to defaults
    // and clears headers and body
    pub fn clear(&mut self) {
        self.proto = Protocol::Http11;
        self.status = status!(200);
        self.headers.clear();
        self.body.clear();
    }

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    /// removes all headers expect for those in excluded
    pub fn clear_headers_exclude(&mut self, excluded: &[&[u8]]) {
        self.headers.retain(|h| excluded.contains(&h.field_ref()))
    }

    /// removes only the headers that are in included
    pub fn clear_headers_include(&mut self, included: &[&[u8]]) {
        self.headers.retain(|h| !included.contains(&h.field_ref()))
    }

    /// removes all headers
    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }
}

#[derive(Debug)]
pub struct ServerRef<'a>(&'a Respond);

impl<'a> core::ops::Deref for ServerRef<'a> {
    type Target = &'a Respond;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ServerRef<'a> {
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
            .copied()
            .chain(self.headers.stream_bytes())
            .chain(Some(10))
            .chain(self.body.as_slice().into_iter().map(|b| *b));

        // TODO needs read to end
        // let _n = self.headers.read(hbuf).unwrap();
        // let stream = stream.chain(hbuf.into_iter().map(|b| *b)).chain(Some(10));
        // let _n = self.body.read(bbuf).unwrap();
        // let stream = stream.chain(bbuf.into_iter().map(|b| *b));

        stream
    }

    pub fn has_body(&self) -> bool {
        !self.body.is_empty()
    }
}

impl Respond {
    pub fn server(self) -> Server {
        Server(self)
    }

    pub fn server_ref<'a, 'b>(&'a self) -> ServerRef<'b>
    where
        'a: 'b,
    {
        ServerRef(self)
    }

    pub fn server_mut<'a, 'b>(&'a mut self) -> ServerMut<'b>
    where
        'a: 'b,
    {
        ServerMut(self)
    }

    pub fn client(self) -> Client {
        Client(self)
    }

    pub fn client_ref<'a, 'b>(&'a self) -> ClientRef<'b>
    where
        'a: 'b,
    {
        ClientRef(self)
    }

    pub fn client_mut<'r, 'c>(&'r mut self) -> ClientMut<'c>
    where
        'r: 'c,
    {
        ClientMut(self)
    }
}
