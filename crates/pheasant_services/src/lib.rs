use pheasant_prologue::{
    ErrorStatus, Method, err_stt,
    server::{Request, Respond},
};

pub mod client_socket;
pub mod content_meta;
pub mod content_security_policy;
pub mod cookies;
pub mod cors;
pub mod errors;
pub mod forward;
pub mod parse;
pub mod print;
pub mod range;
pub mod server_socket;

pub use content_meta::MessageBodyInfo;
pub use content_security_policy::{ContentSecurity, ContentSecurityPolicy};
// pub use content_security_policy::CSP;
pub use cookies::{ReadCookies, WriteCookies};
pub use cors::Cors;
pub use errors::http_error;
pub use forward::Forward;
pub use parse::parse;
pub use range::{Ranges, support_ranges};

type TcpSocket<T> = pheasant_socket::socket::Socket<T>;

pub fn date() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

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
