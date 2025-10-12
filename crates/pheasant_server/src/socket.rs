//! this crate defines the socket api
//!
//! ### APIs
//! - builder
//! - resources
//! - servlets
//! - fallbacks
//! - stream/io

use crate::{Fallback, Request, Resource, Respond, Servlet, request::http11::lex};
use hashbrown::HashSet;
use pheasant_core::{ErrorStatus, Method, Protocol, err_stt};
use pheasant_uri::Scheme;
use std::io::{Read, Result as IoRes, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};

pub mod builder;
pub mod io;

pub use io::{ReceiveStream, SendStream};

// TODO implement Keep-Alive header for http request pipelining

// pub enum SocketError {
//     ReadFailed,
//     WriteFailed,
// }

pub struct HttpSocket {
    /// byte repr of allowed socket protocols ( http1.1, 2, ws,...)
    protos: u8,
    /// the actively used socket protocol
    proto: Protocol,
    /// byte repr of allowed http methods,
    methods: u16,
    /// socket allowed schemes
    scheme: Scheme,
    // tls configuration for use in https requests, if any
    // secure: Option<TlsConfig>,
    /// the tcp listener socket
    socket: TcpListener,
    // the class of the socket, specifies its functionality
    // kind: SocketKind,
    /// set of registered socket services
    resources: HashSet<Resource>,
    // set of registered socket fallbacks (http error status processes)
    fallbacks: HashSet<Fallback>,
    // enables redirects socket wide
    forwarding: bool,
    /// a socket buffer for http io
    primary_buffer: Vec<u8>,
    /// a second socket buffer for http io
    secondary_buffer: Vec<u8>,
    /// a third socket buffer for http io
    tertiary_buffer: Vec<u8>,
    /// max allowed len for request uris
    uri_max: usize,
    /// the max allowed octets size of a header field
    header_max: usize,
    /// the max allowed octets size of all headers fields
    headers_max: usize,
    /// defines the strictness mode of the socket
    ///
    /// strictness is the level of rfc and recency compliance this socket
    /// demands from clients/user agents
    strict: bool,
    /// when this is on, the socket returns an 403 Forbidden error whenever it can/should
    /// instead of returning the real error describing what happened
    /// essentially keeping the user in the dark about the server inner workings
    secretive: bool,
}

// #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
// pub enum SocketKind {
//     #[default]
//     Origin,
//     // Gateway,
//     // Proxy,
//     // LoadBalancer,
// }

impl HttpSocket {
    /// creates a bew HttpSocket
    ///
    /// # Error
    /// - returns an std::io::Error when a valid port is not found after u16::MAX is reached  
    /// - returns an Error if protos is an empty slice
    ///
    /// # Examples
    /// ```
    /// # use pheasant::HttpSocket;
    ///
    /// let (addr, port) = ([127.0.0.1], 8883);
    /// let socket = HttpSocket::new(
    ///     addr, port, None, SocketKind::Origin, Scheme::Http, &[Protocol::Http11]
    /// );
    /// ```
    ///
    // pub fn new(
    //     addr: impl Into<Ipv4Addr>,
    //     port: u16,
    //     tls_config: Option<TlsConfig>,
    //     scheme: Scheme,
    //     protos: &[Protocol],
    //     threads: usize,
    // ) -> IoRes<Self> {
    //     if protos.is_empty() {
    //         return Err(std::io::Error::other(
    //             "http socket needs to support at least 1 protocol",
    //         ));
    //     }
    //
    //     Ok(Self {
    //         // secure: tls_config,
    //         socket: bind_socket(addr, port, scheme)?,
    //         protos: proto_slice_to_u8(protos),
    //         scheme,
    //         fallbacks: HashSet::new(),
    //     })
    // }

    pub fn builder(addr: impl Into<Ipv4Addr>, port: u16) -> IoRes<builder::Builder> {
        let socket = bind_socket(addr, port)?;

        Ok(builder::Builder::new(socket))
    }

    /// returns a result of the origin this socket is bound to
    ///
    /// ### Error
    /// - errors if std::net::SocketAddr.local_addr() returns an error
    ///
    // pub fn origin(&self) -> IoRes<Origin> {
    //     let addr = self.addr()?;
    //     let (ip, port) = (addr.ip(), addr.port());
    //
    //     Ok(Origin::with_port(self.scheme, ip, port))
    // }

    // returns a result of the socket's ip addr
    fn addr(&self) -> IoRes<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn port(&self) -> u16 {
        match self.socket.local_addr() {
            Ok(addr) => addr.port(),
            Err(_) => self.scheme.default_port(),
        }
    }

    /// whether the socket supports secure connections(tls) or not
    ///
    /// > [!WARN]
    /// > tls/https is currently unsupported
    // pub fn is_secure(&self) -> bool {
    //     self.secure.is_some()
    // }

    /// returns this socket's kind
    // pub fn kind(&self) -> SocketKind {
    //     self.kind
    // }

    /// returns a slice of the protocols this socket supports
    ///
    /// > [!WARN]
    /// > currently only recognizes the http1.1 and http2 protocols
    pub fn supported_protocols(&self) -> &[Protocol] {
        match self.protos {
            0 => unreachable!("an empty protocol slice is an error at the Builder level"),
            1 => &[Protocol::Http11],
            2 => &[Protocol::Http2],
            3 => &[Protocol::Http11, Protocol::Http2],
            _ => unreachable!("unrecognized u8 protocols repr"),
        }
    }

    // checks whether this socket supports the http1.1 protocol
    // pub fn supports_http11(&self) -> bool {
    //     self.protos & 1 == Protocol::Http11 as u8
    // }

    // chechs whether this socket supports the http2 protocol
    //
    // > [!WARN]
    // > http2 is yet unsupported
    //
    // pub fn supports_http2(&self) -> bool {
    //     self.protos & 2 == Protocol::Http2 as u8
    // }
}

#[macro_export]
macro_rules! byte_enum_delegate {
    ($field: ident < $ty: ident, $byte: ty> {  $($f: ident: $var: ident),+ }) => {
        $(
            pub fn $f(&mut self, switch: bool) {
                self.$field = if switch {
                    self.$field | ($ty:: $var as $byte)
                } else {
                    self.$field & !($ty:: $var as $byte)
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! byte_enum_match {
    ($field: ident < $ty: ident, $byte: ty> { $($f: ident: $var: ident),+ }) => {
        $(
            pub fn $f(&self) -> bool {
                self.$field & ($ty :: $var as $byte) == ($ty :: $var as $byte)
            }
        )*
    };
}

impl HttpSocket {
    byte_enum_match!(protos<Protocol, u8> { supports_http11: Http11, supports_http2: Http2 });

    byte_enum_match!(methods<Method, u16> {
         supports_get: Get,
         supports_post: Post,
         supports_put: Put,
         supports_patch: Patch,
         supports_delete: Delete,
         supports_options: Options,
         supports_head: Head,
         supports_trace: Trace
    });
}

impl HttpSocket {
    /// registers a new service(s) to this socket
    // pub fn servlet<S, B>(&mut self, s: S) -> &mut Self
    // where
    //     S: Fn() -> B,
    //     B: ServletBundle,
    // {
    //     let bundle = s();
    //     match bundle.size() {
    //         0 => return self,
    //         1 => {
    //             let Some(service) = bundle.iter().next() else {
    //                 unreachable!("size is 1 so we can't fail here");
    //             };
    //
    //             self.servlets.insert(service);
    //         }
    //         _ => self.servlets.extend(bundle.iter()),
    //     }
    //
    //     self
    // }

    // WARN may be faulty
    // never tried in dev or tested
    /// self.service but takes batches of services
    // pub fn servlets<S, B, I>(&mut self, iter: I) -> &mut Self
    // where
    //     S: Fn() -> B,
    //     B: ServletBundle,
    //     I: IntoIterator<Item = S>,
    // {
    //     iter.into_iter().for_each(|s| {
    //         self.service(s);
    //     });
    //
    //     self
    // }

    /// registers a new http failure to this socket
    pub fn fallback<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() -> Fallback,
    {
        self.fallbacks.insert(f());

        self
    }

    // WARN may be faulty
    // never tried in dev or tested
    // / self.failure but takes batches of failures
    // pub fn fallbacks<F, I>(&mut self, iter: I) -> &mut Self
    // where
    //     F: Fn() -> Fallback,
    //     I: IntoIterator<Item = F>,
    // {
    //     iter.into_iter().for_each(|f| {
    //         self.failure(f);
    //     });
    //
    //     self
    // }
}

impl HttpSocket {
    /// returns a shared reference of self
    pub fn as_ref(&self) -> &Self {
        self
    }

    /// returns a mutable borrow of self
    pub fn as_mut(&mut self) -> &mut Self {
        self
    }

    /// returns a shared reference to self.socket (&TcpListener)
    pub fn socket_ref(&self) -> &TcpListener {
        &self.socket
    }

    /// returns a mutable reference to self.socket
    pub fn socket_mut(&mut self) -> &mut TcpListener {
        &mut self.socket
    }

    pub fn init_message(&self) {
        println!(
            "\x1b[1;38;2;111;163;204mSocket listening on http://localhost:{}\x1b[0m",
            self.port()
        );
    }
}

// tries to bind the socket to the passed addr and port
// keeps incrementing port number until it finds a free port
//
// ### Error
// - returns an std::io::Error when port reaches u16::MAX and no free port is found
fn bind_socket(addr: impl Into<Ipv4Addr>, mut port: u16) -> IoRes<TcpListener> {
    let addr = addr.into();
    let socket = loop {
        match TcpListener::bind((addr, port)) {
            Ok(listener) => break listener,
            err if port == u16::MAX => return err,
            _err => port += 1,
        }
    };

    // std::println!(
    //     "\x1b[1;38;2;237;203;244mSocket listening on origin {:?}://{}:{}\x1b[0m",
    //     scheme,
    //     addr,
    //     port
    // );

    Ok(socket)
}

// converts a slice of Protocols to a u8
fn proto_slice_to_u8(protos: &[Protocol]) -> u8 {
    use Protocol::*;

    let mut byte = 0;
    if protos.contains(&Http11) {
        byte |= 1;
    }
    if protos.contains(&Http2) {
        byte |= 2;
    }

    byte
}

#[derive(Debug, Clone, Copy)]
pub struct SocketRef<'a> {
    pub(crate) uri_max: usize,
    pub(crate) header_max: usize,
    pub(crate) headers_max: usize,
    pub(crate) body_max: usize,
    pub(crate) opts: bool,
    pub(crate) head: bool,
    pub(crate) trace: bool,
    pub(crate) resource: &'a Resource,
    pub(crate) methods: u8,
    pub(crate) proto: Protocol,
    pub(crate) protos: u8,
    pub(crate) strict: bool,
}

pub struct ProcessReq<'a> {
    req: Request,
    res: &'a Resource,
    forward: bool,
}

impl<'a> ProcessReq<'a> {
    pub async fn run(self) -> Respond {
        let resp = self.res.process(self.req);

        resp.await
    }
}

pub struct Lookup<'a> {
    res: &'a HashSet<Resource>,
    req: Request,
}

impl<'a> Lookup<'a> {
    pub fn find(self) -> Result<ProcessReq<'a>, ErrorStatus> {
        if let Some(res) = self.route() {
            return Ok(ProcessReq {
                res,
                req: self.req,
                forward: false,
            });
        }

        self.forward()
            .map(|res| ProcessReq {
                res,
                req: self.req,
                forward: true,
            })
            .ok_or_else(|| err_stt!(NotFound))
    }

    pub fn route(&self) -> Option<&'a Resource> {
        self.res
            .into_iter()
            .find(|res| res.resource_for(self.req.route(), self.req.method()))
    }

    pub fn forward(&self) -> Option<&'a Resource> {
        self.res
            .into_iter()
            .find(|res| res.forwards_to(self.req.route(), self.req.method()))
    }
}

type IoErr = std::io::Error;

impl HttpSocket {
    pub fn lookup(&self, req: Request) -> Result<ProcessReq<'_>, ErrorStatus> {
        Lookup {
            res: &self.resources,
            req,
        }
        .find()
    }

    pub fn stream<'a>(&mut self) -> Result<impl Read + Write + use<'a>, std::io::Error> {
        match self.socket.accept() {
            Ok((stream, _)) => Ok(stream),
            Err(e) => Err(e),
        }
    }

    /// generates a new ReceiverStream
    pub fn reader<'a, R: Read>(&'a mut self, stream: &'a mut R) -> ReceiveStream<'a, R> {
        ReceiveStream::new(stream, &mut self.primary_buffer)
    }

    /// receives the request stream and parse it into a request
    pub fn receive<'a, R: Read>(
        &'a mut self,
        stream: &'a mut R,
    ) -> Result<Request, std::io::Error> {
        let recv = self.reader(stream);

        let read = recv
            .recv()
            .map_err(|_| std::io::Error::other("read failed"))?;

        let tokens = lex(&self.primary_buffer[..read], &mut self.secondary_buffer);
        if tokens.is_empty() {
            return err_stt!(?BadRequest).map_err(|_| std::io::Error::other("read failed"));
        }

        Request::parse(tokens).map_err(|_| std::io::Error::other("read failed"))
    }

    /// generates a new SendStream
    pub fn writer<'a, W: Write>(
        &'a mut self,
        stream: &'a mut W,
        amount: usize,
    ) -> SendStream<'a, W> {
        SendStream::new(stream, &mut self.primary_buffer, amount)
    }

    /// sends the respond byte stream back to the client
    pub fn send<W: Write>(&mut self, stream: &mut W, res: Respond) -> Result<(), std::io::Error> {
        let n = res
            .parse(&mut self.primary_buffer)
            .map_err(|_| std::io::Error::other("respond parse failed"))?;
        let send = self.writer(stream, n);

        send.send()
    }

    /// starts up the socket server
    pub async fn fireup(&mut self) -> Result<(), std::io::Error> {
        while let Ok(mut stream) = self.stream() {
            let request = self.receive(&mut stream)?;
            println!("{:#?}", request);
            let process = self
                .lookup(request)
                .map_err(|_| std::io::Error::other("wakanda"))?;
            let respond = process.run().await;
            self.send(&mut stream, respond)?;
        }

        Ok(())
    }
}
