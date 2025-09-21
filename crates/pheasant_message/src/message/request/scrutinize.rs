use crate::{Request, Resource, Scrutinizer};
use hashbrown::HashMap;
use pheasant_core::{ErrorStatus, Method, Protocol, err_stt};

struct ScrutinizeProto<'a>
// where I: Iterator<Item = (&'a str, &'a str)>
{
    pub headers: &'a HashMap<String, String>,
    pub req_proto: Protocol,
    pub proto: Protocol,
    pub protos: u8,
    pub strict: bool,
}

impl<'a> Scrutinizer for ScrutinizeProto<'a> {
    fn scrutinize(&self) -> Result<(), ErrorStatus> {
        // request proto is not within the socket's supported protos
        if self.protos & self.req_proto.to_u8() == 0 {
            return Err(err_stt!(HTTPVersionNotSupported));
        }

        if self.strict {
            if self.req_proto < self.proto {
                return Err(err_stt!(UpgradeRequired));
            }

            if self.headers.contains_key("Upgrade") && self.req_proto > Protocol::HTTP_11 {
                return Err(err_stt!(UnprocessableContent));
            }
        }

        Ok(())
    }
}

struct ScrutinizeHeaders<'a> {
    headers: &'a HashMap<String, String>,
}

impl<'a> Scrutinizer for ScrutinizeHeaders<'a> {
    fn scrutinize(&self) -> Result<(), ErrorStatus> {
        if self.headers.contains_key("Pragma") || self.headers.contains_key("Warning") {
            // not sure what goes here
            return Err(err_stt!(BadRequest));
        }

        Ok(())
    }
}

struct ScrutinizeMethod<'a> {
    headers: &'a HashMap<String, String>,
    method: Method,
    resource: &'a Resource,
    methods: u8,
}

impl<'a> ScrutinizeMethod<'a> {
    fn allows_method(&self) -> bool {
        use Method::*;
        let res = self.resource;

        match self.method {
            Get => res.get.is_some(),
            Post => res.post.is_some(),
            Put => res.put.is_some(),
            Patch => res.patch.is_some(),
            Delete => res.delete.is_some(),
            Options => res.method_is_cross_origin(self.method),
            Head => res.head,
            Trace => res.trace,
            Connect => false,
        }
    }
}

impl<'a> Scrutinizer for ScrutinizeMethod<'a> {
    fn scrutinize(&self) -> Result<(), ErrorStatus> {
        if !self.allows_method() {
            return Err(err_stt!(MethodNotAllowed));
        } else if self.methods & self.method.to_u8() == 0 {
            return Err(err_stt!(NotImplemented));
        }

        Ok(())
    }
}

struct ScrutinizeSocketSizes {
    socket_body_max: usize,
    socket_header_max: usize,
    socket_headers_max: usize,
    socket_uri_max: usize,
    req_body: usize,
    req_header: usize,
    req_headers: usize,
    req_uri: usize,
}

impl Scrutinizer for ScrutinizeSocketSizes {
    fn scrutinize(&self) -> Result<(), ErrorStatus> {
        if self.socket_uri_max < self.req_uri {
            return Err(err_stt!(URITooLong));
        } else if self.socket_body_max < self.req_body {
            return Err(err_stt!(ContentTooLarge));
        } else if self.socket_header_max < self.req_header {
            return Err(err_stt!(RequestHeaderFieldsTooLarge));
        } else if self.socket_headers_max < self.req_headers {
            return Err(err_stt!(RequestHeaderFieldsTooLarge));
        }

        Ok(())
    }
}

fn scrutinize(req: &Request, socket: SocketRef<'_>) -> Result<(), ErrorStatus> {
    ScrutinizeSocketSizes::new().scrutinize()?;
    ScrutinizeMethod::new()?.scrutinize();
    ScrutinizeProto::new()?.scrutinize();
    ScrutinizeHeaders::new()?.scrutinize();

    Ok(())
}
