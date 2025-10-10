use super::HttpSocket;
use crate::byte_enum_delegate;
use crate::socket::{Fallback, Resource, Servlet};
use hashbrown::HashSet;
use pheasant_core::{Method, Protocol};
use pheasant_uri::Scheme;
use std::net::TcpListener;

// #[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Builder<const BUF_SIZE: usize> {
    protos: u8,
    proto: Protocol,
    methods: u16,
    scheme: Scheme,
    socket: TcpListener,
    forwarding: bool,
    uri_upper_size: usize,
    header_upper_size: usize,
    headers_upper_size: usize,
    strict: bool,
    secretive: bool,
    resources: HashSet<Resource>,
    fallbacks: HashSet<Fallback>,
}

impl<const BUF_SIZE: usize> Builder<BUF_SIZE> {
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

impl<const BUF_SIZE: usize> Builder<BUF_SIZE> {
    pub fn new(socket: TcpListener) -> Self {
        Self {
            socket,
            protos: 1,
            proto: Protocol::Http11,
            methods: 6,
            scheme: Scheme::Http,
            forwarding: false,
            uri_upper_size: 1024,
            headers_upper_size: 2048,
            header_upper_size: 256,
            strict: true,
            secretive: true,
            resources: HashSet::new(),
            fallbacks: HashSet::new(),
        }
    }

    pub fn build(self) -> Result<HttpSocket<BUF_SIZE>, Error> {
        if self.header_upper_size == 0
            || self.headers_upper_size == 0
            || BUF_SIZE == 0
            || self.uri_upper_size == 0
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
            uri_upper_size: self.uri_upper_size,
            header_upper_size: self.header_upper_size,
            headers_upper_size: self.headers_upper_size,
            forwarding: self.forwarding,
            strict: self.strict,
            secretive: self.secretive,
            buffer: [0; BUF_SIZE],
        })
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

    pub fn uri_upper_size(mut self, upper: usize) -> Self {
        self.uri_upper_size = upper;

        self
    }

    pub fn header_upper_size(mut self, upper: usize) -> Self {
        self.header_upper_size = upper;

        self
    }

    pub fn headers_upper_size(mut self, upper: usize) -> Self {
        self.headers_upper_size = upper;

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
