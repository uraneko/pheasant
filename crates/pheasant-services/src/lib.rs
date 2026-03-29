// #![no_std]
// #![forbid(clippy::unwrap_used, clippy::expect_used)]
use embedded_io::{Read, Write};
use pheasant_http::{ErrorStatus, Header, Method, err_stt};

pub mod content;
pub mod cookies;
pub mod cors;
pub mod errors;
pub mod forward;
pub mod gateway;
pub mod parse;
pub mod print;
pub mod range;
pub mod socket;

pub use content::Content;
pub use cookies::{ReadCookies, WriteCookies};
pub use cors::Cors;
pub use errors::http_error;
pub use forward::Forward;
pub use gateway::{Blacklist, GateWay, Whitelist};
pub use parse::{request, respond};
pub use range::{Ranges, support_ranges};

type TcpSocket<T> = pheasant_socket::socket::Socket<T>;

pub fn date() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

type Request = pheasant_http::Request<Vec<Header>>;
type Respond = pheasant_http::Respond<Vec<u8>>;

pub trait Service<S: Server> {
    async fn serve(
        &self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus>;
}

pub trait Server {
    async fn event_loop(
        &mut self,
        fun: impl AsyncFn(&mut Self) -> Result<(), ErrorStatus>,
    ) -> Result<(), ErrorStatus> {
        fun(self).await
    }

    // core services are run through the socket
    // this would ofc mean that the socket stores the server state
    // or whatever state is needed by the services to run correctly
    async fn service(
        &mut self,
        req: Request,
        buf: &mut Respond,
        service: impl Service<Self>,
    ) -> Result<(), ErrorStatus>
    where
        Self: Sized,
    {
        service.serve(self, req, buf).await
    }
}

pub trait Resource<S: Server>: Sized {
    async fn get(
        self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn post(
        self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn put(
        self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn patch(
        self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn head(
        self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn trace(
        self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn delete(
        self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn options(
        self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn connect(
        self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn run(
        self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        use Method::*;

        match req.method() {
            Get => self.get(socket, req, resp).await,
            Post => self.post(socket, req, resp).await,
            Head => self.head(socket, req, resp).await,
            Patch => self.patch(socket, req, resp).await,
            Put => self.put(socket, req, resp).await,
            Connect => self.connect(socket, req, resp).await,
            Options => self.options(socket, req, resp).await,
            Delete => self.delete(socket, req, resp).await,
            Trace => self.trace(socket, req, resp).await,
        }
    }
}

// pub trait Requester {
//     type Method;
//     type Protocol;
//     type Header;
//     type Body;
//
//     pub fn method(&self) -> &Self::Method;
//
//     pub fn protocol(&self) -> &Self::Protocol;
//
//     pub fn header(&self)
// }
//
// pub trait Respondent {}
