use crate::message::{
    Token,
    http11::{Error, Lex, build_headers, content_length},
};
use crate::{Header, Headers, Method, Protocol};
use alloc::string::String;
use alloc::vec::Vec;
use pheasant_uri::{Path, Query};

#[derive(Debug, Clone)]
pub struct Request {
    pub(crate) method: Method,
    pub(crate) path: Path,
    pub(crate) query: Option<Query>,
    pub(crate) proto: Protocol,
    pub(crate) headers: Headers,
    pub(crate) body: Option<Vec<u8>>,
}

impl Request {
    pub fn new(method: Method, path: Path, proto: Protocol) -> Self {
        Request {
            method,
            path,
            proto,
            query: None,
            headers: Headers::default(),
            body: None,
        }
    }
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
        let headers = build_headers(headers)?.into();

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

#[derive(Debug, Clone)]
pub struct Server(Request);

impl core::ops::Deref for Server {
    type Target = Request;

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

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_ref().map(|b| b.as_slice())
    }

    pub fn take_path(&mut self) -> Vec<String> {
        self.path.take_segments()
    }

    pub fn query_mut(&mut self) -> Option<&mut Query> {
        self.query.as_mut()
    }

    pub fn take_body(&mut self) -> Option<Vec<u8>> {
        core::mem::take(&mut self.body)
    }

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
    /// use pheasant_http::message::http11::{Error, Lex};
    ///
    /// let mut req = Lex::new(b"GET / HTTP/1.1\naccess-control-request-method: GET\naccess-control-request-header: ranges\norigin: localhost\n\n").request()?;
    /// req.server_mut().lowercase_header_value(b"access-control-request-headers");
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
pub struct ServerMut<'a>(&'a mut Request);

impl core::ops::Deref for ServerMut<'_> {
    type Target = Request;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for ServerMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct ServerRef<'a>(&'a Request);

impl core::ops::Deref for ServerRef<'_> {
    type Target = Request;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ServerRef<'a> {
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

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_ref().map(|b| b.as_slice())
    }
}

impl<'a> ServerMut<'a> {
    pub fn take_path(&mut self) -> Vec<String> {
        self.path.take_segments()
    }

    pub fn query_mut(&mut self) -> Option<&mut Query> {
        self.query.as_mut()
    }

    pub fn take_body(&mut self) -> Option<Vec<u8>> {
        core::mem::take(&mut self.body)
    }

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
    /// use pheasant_http::message::http11::{Error, Lex};
    ///
    /// let mut req = Lex::new(b"GET / HTTP/1.1\naccess-control-request-method: GET\naccess-control-request-header: ranges\norigin: localhost\n\n").request()?;
    /// req.server_mut().lowercase_header_value(b"access-control-request-headers");
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
pub struct ClientMut<'a>(&'a mut Request);

impl<'a> core::ops::Deref for ClientMut<'a> {
    type Target = &'a mut Request;

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
    pub fn query_mut(&mut self) -> Option<&mut Query> {
        self.query.as_mut()
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn body_mut(&mut self) -> Option<&mut Vec<u8>> {
        self.body.as_mut()
    }

    pub fn clear(&mut self) {
        self.method = Method::Get;
        self.headers.clear();
        if let Some(ref mut body) = self.body {
            body.clear();
        }
    }

    pub fn clear_body(&mut self) {
        self.body = None;
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

#[derive(Debug, Clone)]
pub struct ClientRef<'a>(&'a Request);

impl<'a> core::ops::Deref for ClientRef<'a> {
    type Target = &'a Request;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ClientRef<'a> {
    pub fn path_str(&self) -> alloc::string::String {
        self.path.serialized()
    }

    pub fn stream_bytes(&self) -> impl IntoIterator<Item = u8> {
        let q = self
            .query
            .as_ref()
            .map(|q| q.to_bytes())
            .unwrap_or_else(|| Vec::new());

        let b = self
            .body
            .as_ref()
            .map(|b| b.clone())
            .unwrap_or_else(|| Vec::new());

        self.method
            .as_bytes()
            .into_iter()
            .chain(Some(&32))
            .copied()
            .chain(self.path.serialized().into_bytes())
            .chain(q)
            .chain(Some(32))
            .chain(self.proto.as_bytes().into_iter().map(|b| *b))
            .chain(Some(10))
            .chain(self.headers.stream_bytes())
            .chain(Some(10))
            .chain(b)
    }
}

#[derive(Debug, Clone)]
pub struct Client(Request);

impl core::ops::Deref for Client {
    type Target = Request;

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
    pub fn method<M>(mut self, method: M) -> Result<Self, M::Error>
    where
        M: TryInto<Method>,
    {
        self.method = method.try_into()?;

        Ok(self)
    }

    pub fn proto<P>(mut self, proto: P) -> Result<Self, P::Error>
    where
        P: TryInto<Protocol>,
    {
        self.proto = proto.try_into()?;

        Ok(self)
    }

    pub fn path<P>(mut self, path: P) -> Result<Self, P::Error>
    where
        P: TryInto<Path>,
    {
        self.path = path.try_into()?;

        Ok(self)
    }

    pub fn path_str(&self) -> alloc::string::String {
        self.path.serialized()
    }

    pub fn stream_bytes(&self) -> impl IntoIterator<Item = u8> {
        let q = self
            .query
            .as_ref()
            .map(|q| q.to_bytes())
            .unwrap_or_else(|| Vec::new());

        let b = self
            .body
            .as_ref()
            .map(|b| b.clone())
            .unwrap_or_else(|| Vec::new());

        self.method
            .as_bytes()
            .into_iter()
            .chain(Some(&32))
            .copied()
            .chain(self.path.serialized().into_bytes())
            .chain(q)
            .chain(Some(32))
            .chain(self.proto.as_bytes().into_iter().map(|b| *b))
            .chain(Some(10))
            .chain(self.headers.stream_bytes())
            .chain(Some(10))
            .chain(b)
    }

    pub fn query_mut(&mut self) -> Option<&mut Query> {
        self.query.as_mut()
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn body_mut(&mut self) -> Option<&mut Vec<u8>> {
        self.body.as_mut()
    }

    pub fn clear(&mut self) {
        self.method = Method::Get;
        self.headers.clear();
        if let Some(ref mut body) = self.body {
            body.clear();
        }
    }

    pub fn clear_body(&mut self) {
        self.body = None;
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

impl Request {
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

    pub fn client_mut<'a, 'b>(&'a mut self) -> ClientMut<'b>
    where
        'a: 'b,
    {
        ClientMut(self)
    }
}
