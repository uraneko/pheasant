use super::HttpSocket;
use crate::byte_enum_delegate;
use crate::socket::{Fallback, Resource, Servlet};
use hashbrown::HashSet;
use pheasant_core::{Method, Protocol};
use pheasant_uri::Scheme;
use std::net::TcpListener;

// #[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Builder {
    protos: u8,
    proto: Protocol,
    methods: u16,
    scheme: Scheme,
    socket: TcpListener,
    forwarding: bool,
    buf_size: usize,
    uri_max: usize,
    header_max: usize,
    headers_max: usize,
    body_max: usize,
    strict: bool,
    secretive: bool,
    resources: HashSet<Resource>,
    fallbacks: HashSet<Fallback>,
}

impl Builder {
    byte_enum_delegate!(protos<Protocol, u8> { http11: Http11, http2: Http2 });
    byte_enum_delegate!(methods<Method, u16> {
        get: Get,
        post: Post,
        put: Put,
        patch: Patch,
        delete: Delete,
        options: Options,
        head: Head,
        trace: Trace
    });
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    UpperSizesCanNotBeNull,
    MustServeAtLeastOneResource,
}

impl Builder {
    pub fn new(socket: TcpListener) -> Self {
        Self {
            socket,
            protos: 1,
            proto: Protocol::Http11,
            methods: 6,
            scheme: Scheme::Http,
            forwarding: false,
            buf_size: 4069,
            uri_max: 256,
            headers_max: 2048,
            header_max: 256,
            body_max: 4096,
            strict: true,
            secretive: true,
            resources: HashSet::new(),
            fallbacks: HashSet::new(),
        }
    }

    pub fn build(self) -> Result<HttpSocket, Error> {
        if self.header_max == 0 || self.headers_max == 0 || self.buf_size == 0 || self.uri_max == 0
        {
            return Err(Error::UpperSizesCanNotBeNull);
        }

        if self.resources.is_empty() {
            return Err(Error::MustServeAtLeastOneResource);
        }

        Ok(HttpSocket {
            protos: self.protos,
            methods: self.methods,
            socket: self.socket,
            scheme: self.scheme,
            proto: self.proto,
            fallbacks: self.fallbacks,
            resources: self.resources,
            uri_max: self.uri_max,
            header_max: self.header_max,
            headers_max: self.headers_max,
            body_max: self.body_max,
            forwarding: self.forwarding,
            strict: self.strict,
            secretive: self.secretive,
            buffer: Vec::with_capacity(self.buf_size),
        })
    }

    pub fn buf_size(mut self, size: usize) -> Self {
        self.buf_size = size;

        self
    }

    pub fn forwarding(mut self, bool: bool) -> Self {
        self.forwarding = bool;

        self
    }

    pub fn strict(mut self, bool: bool) -> Self {
        self.strict = bool;

        self
    }

    pub fn secretive(mut self, bool: bool) -> Self {
        self.secretive = bool;

        self
    }

    pub fn uri_max(mut self, upper: usize) -> Self {
        self.uri_max = upper;

        self
    }

    pub fn header_max(mut self, upper: usize) -> Self {
        self.header_max = upper;

        self
    }

    pub fn headers_max(mut self, upper: usize) -> Self {
        self.headers_max = upper;

        self
    }

    pub fn body_max(mut self, upper: usize) -> Self {
        self.body_max = upper;

        self
    }

    pub fn resource(mut self, resource: Resource) -> Self {
        self.resources.insert(resource);

        self
    }

    pub fn resources(mut self, resources: impl IntoIterator<Item = Resource>) -> Self {
        self.resources.extend(resources);

        self
    }

    pub fn fallback(mut self, fallback: Fallback) -> Self {
        self.fallbacks.insert(fallback);

        self
    }

    pub fn fallbacks(mut self, fallbacks: impl IntoIterator<Item = Fallback>) -> Self {
        self.fallbacks.extend(fallbacks);

        self
    }

    pub fn scheme(mut self, scheme: Scheme) -> Self {
        self.scheme = scheme;

        self
    }

    pub fn proto(mut self, protocol: Protocol) -> Self {
        self.proto = protocol;

        self
    }
}
