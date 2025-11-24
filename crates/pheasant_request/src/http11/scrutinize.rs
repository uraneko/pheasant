use crate::{HttpSocket, Request, Scrutinizer, byte_enum_match};
use pheasant_core::{ErrorStatus, Method, Protocol, err_stt};
use pheasant_middleware::Headers;

pub struct ScrutinizeProto<'a>
// where I: Iterator<Item = (&'a str, &'a str)>
{
    /// req headers
    pub headers: &'a Headers,
    /// req protocol
    pub req_proto: Protocol,
    /// current socket protocol
    pub proto: Protocol,
    /// supported socket protocols
    pub protos: u8,
    /// is socket in strict mode
    pub strict: bool,
}

impl<'a> ScrutinizeProto<'a> {
    pub fn new(req: &'a Request, socket: &HttpSocket) -> Self {
        Self {
            protos: socket.protos,
            strict: socket.strict,
            proto: socket.proto,
            req_proto: req.proto,
            headers: &req.headers,
        }
    }
}

impl<'a> Scrutinizer for ScrutinizeProto<'a> {
    fn scrutinize(self) -> Result<(), ErrorStatus> {
        // request proto is not within the socket's supported protos
        if self.protos & self.req_proto as u8 == 0 {
            return Err(err_stt!(HTTPVersionNotSupported));
        }

        if self.strict {
            if self.req_proto < self.proto {
                return err_stt!(?UpgradeRequired);
            }

            if self.headers.contains("Upgrade") && self.req_proto > Protocol::Http11 {
                return err_stt!(?UnprocessableContent);
            }
        }

        Ok(())
    }
}

pub struct ScrutinizeHeaders<'a> {
    headers: &'a Headers,
}

impl<'a> ScrutinizeHeaders<'a> {
    pub fn new(request: &'a Request) -> Self {
        Self {
            headers: &request.headers,
        }
    }
}

impl<'a> Scrutinizer for ScrutinizeHeaders<'a> {
    fn scrutinize(self) -> Result<(), ErrorStatus> {
        if self.headers.contains("Pragma") || self.headers.contains("Warning") {
            // not sure what goes here
            return err_stt!(?BadRequest);
        }

        Ok(())
    }
}

pub struct ScrutinizeMethod<'a> {
    /// req headers
    headers: &'a Headers,
    /// req method
    method: Method,
    /// socket supported methods
    methods: u16,
    /// endpoint cors methods
    cors_methods: Vec<Method>,
    /// endpoint supported methods
    res_methods: Vec<Method>,
}

impl<'a> ScrutinizeMethod<'a> {
    pub fn new(
        request: &'a Request,
        socket: &HttpSocket,
        cors_methods: Vec<Method>,
        res_methods: Vec<Method>,
    ) -> Self {
        Self {
            headers: &request.headers,
            method: request.method,
            cors_methods,
            res_methods,
            methods: socket.methods,
        }
    }
}

impl<'a> ScrutinizeMethod<'a> {
    byte_enum_match!(method<Method, u16> {
        supports_get: Get,
        supports_post: Post,
        supports_put: Put,
        supports_patch: Patch,
        supports_delete: Delete,
        supports_options: Options,
        supports_head: Head,
        supports_trace: Trace
    });

    fn allows_method(&self) -> bool {
        use Method::*;
        let res = &self.res_methods;
        let cors = &self.cors_methods;

        match self.method {
            Get => res.contains(&Get),
            Post => res.contains(&Post),
            Put => res.contains(&Put),
            Patch => res.contains(&Patch),
            Delete => res.contains(&Delete),
            Options => !self.supports_options() || cors.contains(&Options),
            Head => !self.supports_head() || res.contains(&Head),
            Trace => !self.supports_trace() || res.contains(&Trace),
            // Connect is for proxies only
            // this framework only supports origin servers for now
            Connect => false,
        }
    }
}

impl<'a> Scrutinizer for ScrutinizeMethod<'a> {
    fn scrutinize(self) -> Result<(), ErrorStatus> {
        if !self.allows_method() {
            return err_stt!(?MethodNotAllowed);
        } else if self.methods & self.method as u16 == 0 {
            return err_stt!(?NotImplemented);
        }

        Ok(())
    }
}

pub struct ScrutinizeSocketSizes {
    socket_body_max: usize,
    socket_header_max: usize,
    socket_headers_max: usize,
    socket_uri_max: usize,
    req_body: usize,
    req_header: usize,
    req_headers: usize,
    req_uri: usize,
}

impl ScrutinizeSocketSizes {
    pub fn new(req: &Request, socket: &HttpSocket) -> Self {
        Self {
            socket_body_max: socket.body_max,
            socket_header_max: socket.header_max,
            socket_headers_max: socket.headers_max,
            socket_uri_max: socket.uri_max,
            req_body: req.body.as_ref().map(|b| b.len()).unwrap_or_else(|| 0),
            req_header: req
                .headers
                .iter()
                .map(|(_, h)| h.len())
                .max()
                .unwrap_or_else(|| 0),
            req_headers: req.headers.iter().map(|(_, h)| h.len()).sum(),
            req_uri: req.route.len(),
        }
    }
}

impl Scrutinizer for ScrutinizeSocketSizes {
    fn scrutinize(self) -> Result<(), ErrorStatus> {
        if self.socket_uri_max < self.req_uri {
            return Err(err_stt!(URITooLong));
        } else if self.socket_body_max < self.req_body {
            return Err(err_stt!(ContentTooLarge));
        } else if self.socket_header_max < self.req_header {
            return err_stt!(?RequestHeaderFieldsTooLarge);
        } else if self.socket_headers_max < self.req_headers {
            return err_stt!(?RequestHeaderFieldsTooLarge);
        }

        Ok(())
    }
}

use std::io::Result as IoRes;
use std::net::{Ipv4Addr, TcpListener};

// tries to bind the socket to the passed addr and port
// keeps incrementing port number until it finds a free port
//
// ### Error
// - returns an std::io::Error when port reaches u16::MAX and no free port is found
pub fn bind_socket(addr: impl Into<Ipv4Addr>, mut port: u16) -> IoRes<TcpListener> {
    let addr = addr.into();
    let socket = loop {
        match TcpListener::bind((addr, port)) {
            Ok(listener) => break listener,
            err if port == u16::MAX => return err,
            _err => port += 1,
        }
    };

    Ok(socket)
}
