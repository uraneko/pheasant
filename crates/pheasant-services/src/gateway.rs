use crate::respond;
use crate::socket::client::{Error as ClientError, Socket};
use hashbrown::HashSet;
use pheasant_prologue::client::{Request, Respond};
use pheasant_socket::address::SockAddrIn;

pub enum Error {
    NoOriginHeader,
    ParseFailed,
    Client(ClientError),
}

pub struct GateWay;

impl GateWay {
    /// routes the request to the appropriate service,
    /// if the request is for the gateway server itself
    /// then None is returned
    /// else a service address is returned
    pub fn route<'a>(
        router: impl Fn(&str) -> Option<&str>,
        path: &'a str,
    ) -> Result<Option<&'a str>, Error> {
        Ok(router(path))
    }

    /// pings all of the gateway's services by sending a `Head / http1.1`
    /// request to each of them
    pub fn ping() -> Result<(), Error> {
        todo!();

        Ok(())
    }

    /// sends the request to the appropriate service and gets back the result
    /// use client.connect(result of self.route) before calling this
    pub fn service(service: &mut Socket, req: Request) -> Result<Respond, Error> {
        let _n = service.write(req)?;
        let n = service.read()?;
        let resp = &service.buf_ref()[..n];
        let resp = respond(resp).map_err(|_| Error::ParseFailed)?;
        // let n = service.write(req)?;
        // let n = service.read()?;
        //
        // parse(service.buf_ref()[..n])?
        Ok(resp)
    }
}

impl From<ClientError> for Error {
    fn from(err: ClientError) -> Self {
        Self::Client(err)
    }
}

pub struct Whitelist {
    addrs: HashSet<SockAddrIn>,
}

impl Whitelist {
    pub fn new() -> Self {
        Self {
            addrs: HashSet::new(),
        }
    }

    pub fn addr(mut self, addr: &str) -> Result<Self, Error> {
        self.addrs
            .insert(addr.parse().map_err(|_| Error::ParseFailed)?);

        Ok(self)
    }

    /// returning true means the address is whitelisted
    ///
    /// whitelisting is exclusive: all addresses are unallowed expect for those in the
    /// whitelist
    pub fn guard(&self, socket: &pheasant_socket::socket::Socket<SockAddrIn>) -> bool {
        let sockaddr = socket.sockaddr();

        self.addrs.contains(sockaddr)
    }
}

pub struct Blacklist {
    addrs: HashSet<SockAddrIn>,
}

impl Blacklist {
    pub fn new() -> Self {
        Self {
            addrs: HashSet::new(),
        }
    }

    pub fn addr(mut self, addr: &str) -> Result<Self, Error> {
        self.addrs
            .insert(addr.parse().map_err(|_| Error::ParseFailed)?);

        Ok(self)
    }

    /// returning true means the address is blacklisted
    ///
    /// blacklisting is inclusive: all addresses are allowed except for those in the
    /// blacklist
    pub fn guard(&self, socket: &pheasant_socket::socket::Socket<SockAddrIn>) -> bool {
        let sockaddr = socket.sockaddr();

        self.addrs.contains(sockaddr)
    }
}
