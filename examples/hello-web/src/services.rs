use pheasant::prologue::{
    ErrorStatus, err_stt,
    server::{Request, Respond},
};
use pheasant::services::{MessageBodyInfo, Resource, Service, server_socket::Socket};

#[derive(Debug)]
pub enum Services {
    Hello,
    BinUp,
}

impl Service<Socket> for Services {
    async fn serve(
        &self,
        socket: &mut Socket,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        match self {
            Self::Hello => Hello::new(&req).run(socket, req, resp).await,
            Self::BinUp => BinUp.run(socket, req, resp).await,
        }
    }
}

pub struct Hello(String);
// nice crimsonish color #ab1243

impl Hello {
    fn new(req: &Request) -> Self {
        let segments = req.path();
        match segments.last().map(|s: &String| s.as_str()) {
            // WARN None should actually be unreachable from here
            Some("hello") | None => return Self("kappa".into()),
            Some(who) => Self(who.into()),
        }
    }
}

impl Resource<Socket> for Hello {
    async fn get(
        self,
        _socket: &mut Socket,
        _req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        resp.body_mut().extend(b"Hello ");
        resp.body_mut().extend(self.0.as_bytes());
        MessageBodyInfo::new(resp.body_ref()).dump_headers(resp.headers_mut());

        Ok(())
    }
}

// slice!("HTTP/1.1 200 OK\ncontent-length: {}\n\n{}", len, data)

const BINUP_ICON: &[u8] = include_bytes!("../bin-up.svg");

pub struct BinUp;

impl Resource<Socket> for BinUp {
    async fn get(
        self,
        _socket: &mut Socket,
        _req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        resp.body_mut().extend(BINUP_ICON);
        MessageBodyInfo::new(BINUP_ICON).dump_headers(resp.headers_mut());

        Ok(())
    }
}

pub fn lookup(path: &str) -> Result<Services, ErrorStatus> {
    if path.starts_with("/hello") {
        Ok(Services::Hello)
    } else if path.starts_with("/binup") {
        Ok(Services::BinUp)
    } else {
        err_stt!(?404)
    }
}
