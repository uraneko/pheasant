use crossbeam_utils::thread;
use hashbrown::{HashMap, HashSet};
use pheasant_core::Protocol;
use pheasant_message::{Fallback, Process, ProcessBundle};
use pheasant_uri::{Origin, Scheme};
use std::io::Result as IOResult;
use std::net::{Ipv4Addr, SocketAddr, TcpListener};

// TODO implement Keep-Alive header for http request pipelining

pub struct HttpSocket<const BUF_SIZE: usize> {
    /// byte repr of allowed socket protocols ( http1.1, 2, ws,...)
    /// must match scheme
    protos: u8,
    /// byte repr of allowed http methods,
    methods: u8,
    /// tls configuration for use in https requests, if any
    // TODO
    // secure: Option<TlsConfig>,
    /// the tcp listener socket
    socket: TcpListener,
    /// the class of the socket, specifies its functionality
    kind: SocketKind,
    // set of registered socket services
    resources: HashSet<Resource>,
    // set of registered socket fallbacks (http error status processes)
    fallbacks: HashSet<Fallback>,
    // socket origin scheme part
    scheme: Scheme,
    // enables redirects socket wide
    allow_forwarding: bool,
    /// enables Options method socket wide
    /// when this is off all cross origin requests will be rejected with 400 Unauthorized
    allow_options: bool,
    /// enables Trace method socket wide
    /// when this is off all Trace requests will be rejected with 403 Forbidden
    allow_trace: bool,
    /// enables Head method socket wide
    allow_head: bool,
    /// the socket buffer for http io
    buffer: [u8; BUF_SIZE],
    /// max allowed len for request uris
    uri_upper_size: usize,
    /// the max allowed octets size of a header field
    header_upper_size: usize,
    /// the max allowed octets size of all headers fields
    headers_upper_size: usize,
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

pub struct Builder {}

impl Builder {
    fn build(self) -> Result<HttpSocket, PheasantError> {}
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SocketKind {
    #[default]
    Origin,
    // Gateway,
    // Proxy,
    // LoadBalancer,
}

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
    ///     addr, port, None, SocketKind::Origin, Scheme::Http, &[Protocol::HTTP11]
    /// );
    /// ```
    ///
    pub fn new(
        addr: impl Into<Ipv4Addr>,
        port: u16,
        tls_config: Option<TlsConfig>,
        kind: SocketKind,
        scheme: Scheme,
        protos: &[Protocol],
        threads: usize,
    ) -> IOResult<Self> {
        if protos.is_empty() {
            return Err(std::io::Error::other(
                "http socket needs to support at least 1 protocol",
            ));
        }

        Ok(Self {
            // secure: tls_config,
            socket: bind_socket(addr, port, scheme)?,
            protos: proto_slice_to_u8(protos),
            kind,
            scheme,
            fallbacks: HashSet::new(),
        })
    }

    /// returns a result of the origin this socket is bound to
    ///
    /// ### Error
    /// - errors if std::net::SocketAddr.local_addr() returns an error
    ///
    pub fn origin(&self) -> IOResult<Origin> {
        let addr = self.addr()?;
        let (ip, port) = (addr.ip(), addr.port());

        Ok(Origin::from_parts(self.scheme, ip, port))
    }

    // returns a result of the socket's ip addr
    fn addr(&self) -> IOResult<SocketAddr> {
        self.socket.local_addr()
    }

    /// whether the socket supports secure connections(tls) or not
    ///
    /// > [!WARN]
    /// > tls/https is currently unsupported
    pub fn is_secure(&self) -> bool {
        self.secure.is_some()
    }

    /// returns this socket's kind
    pub fn kind(&self) -> SocketKind {
        self.kind
    }

    /// returns a slice of the protocols this socket supports
    ///
    /// > [!WARN]
    /// > currently only recognizes the http1.1 and http2 protocols
    pub fn supported_protocols(&self) -> &[Protocol] {
        match self.protos {
            0 => unreachable!("an empty protocol slice is an error at HttpSocket::new"),
            1 => &[Protocol::HTTP11],
            2 => &[Protocol::HTTP2],
            3 => &[Protocol::HTTP11, Protocol::HTTP2],
            _ => unreachable!("unrecognized u8 protocols repr"),
        }
    }

    /// checks whether this socket supports the http1.1 protocol
    pub fn supports_http11(&self) -> bool {
        self.protos & 1 == 1
    }

    /// chechs whether this socket supports the http2 protocol
    ///
    /// > [!WARN]
    /// > http2 is yet unsupported
    ///
    pub fn supports_http2(&self) -> bool {
        self.protos & 2 == 2
    }
}

impl HttpSocket {
    /// registers a new service(s) to this socket
    pub fn service<S, B>(&mut self, s: S) -> &mut Self
    where
        S: Fn() -> B,
        B: ProcessBundle,
    {
        let bundle = s();
        match bundle.size() {
            0 => return self,
            1 => {
                let Some(service) = bundle.iter().next() else {
                    unreachable!("size is 1 so we can't fail here");
                };

                self.services.insert(service);
            }
            _ => self.services.extend(bundle.iter()),
        }

        self
    }

    // WARN may be faulty
    // never tried in dev or tested
    /// self.service but takes batches of services
    pub fn services<S, B, I>(&mut self, iter: I) -> &mut Self
    where
        S: Fn() -> B,
        B: ProcessBundle,
        I: IntoIterator<Item = S>,
    {
        iter.into_iter().for_each(|s| {
            self.service(s);
        });

        self
    }

    /// registers a new http failure to this socket
    pub fn failure<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() -> Fallback,
    {
        self.failures.insert(f());

        self
    }

    // WARN may be faulty
    // never tried in dev or tested
    /// self.failure but takes batches of failures
    pub fn failures<F, I>(&mut self, iter: I) -> &mut Self
    where
        F: Fn() -> Fallback,
        I: IntoIterator<Item = F>,
    {
        iter.into_iter().for_each(|f| {
            self.failure(f);
        });

        self
    }
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

    /// returns an iterator to shared references of self.services items
    pub fn services_iter(&self) -> impl Iterator<Item = &Process> {
        self.services.iter()
    }

    /// returns an iterator to shared references of self.failures items
    pub fn failures_iter(&self) -> impl Iterator<Item = &Fallback> {
        self.failures.iter()
    }
}

// tries to bind the socket to the passed addr and port
// keeps incrementing port number until it finds a free port
//
// ### Error
// - returns an std::io::Error when port reaches u16::MAX and no free port is found
fn bind_socket(addr: impl Into<Ipv4Addr>, mut port: u16, scheme: Scheme) -> IOResult<TcpListener> {
    let addr = addr.into();
    let socket = loop {
        match TcpListener::bind((addr, port)) {
            Ok(listener) => break listener,
            err if port == u16::MAX => return err,
            _err => port += 1,
        }
    };

    std::println!(
        "\x1b[1;38;2;237;203;244mSocket listening on origin {:?}://{}:{}\x1b[0m",
        scheme,
        addr,
        port
    );

    Ok(socket)
}

// converts a slice of Protocols to a u8
fn proto_slice_to_u8(protos: &[Protocol]) -> u8 {
    use Protocol::*;

    let mut byte = 0;
    if protos.contains(&HTTP11) {
        byte |= 1;
    }
    if protos.contains(&HTTP2) {
        byte |= 2;
    }

    byte
}
