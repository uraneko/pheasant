use pheasant_http::{ErrorStatus, Method, Respond, err_stt, request::Request};

pub mod content_meta;
pub mod cookies;
pub mod cors;
pub mod errors;
pub mod parse;
pub mod range;
pub mod server_socket;
pub mod stream;

pub use content_meta::MessageBodyInfo;
pub use cookies::{ReadCookies, WriteCookies};
pub use cors::Cors;
pub use errors::{bad_request, error_status, internal_server_error, not_found};
pub use parse::parse;
pub use range::{Ranges, support_ranges};
pub use server_socket::{Socket, bind_socket};
pub use stream::{read_stream, req_buf, resp_write_stream, write_stream};

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

    /// prints out the socket url on the stdout
    fn init_message(&self) {
        use std::io::Write;

        let mut stdout = std::io::stdout();
        _ = stdout.write(
            format!(
                "\x1b[1;38;2;211;163;104mSocket listening on http://{}\x1b[0m\r\n",
                self.addr()
                    .map(|addr| addr.to_string())
                    .unwrap_or(format!("localhost:{}", self.port())),
            )
            .as_bytes(),
        );
        _ = stdout.flush();
    }

    fn addr(&self) -> Result<std::net::SocketAddr, std::io::Error>;

    fn port(&self) -> u16;
}

pub trait Resource<S: Server> {
    async fn get(
        &self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn post(
        &self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn put(
        &self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn patch(
        &self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn head(
        &self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn trace(
        &self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn delete(
        &self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn options(
        &self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn connect(
        &self,
        socket: &mut S,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        err_stt!(?405)
    }

    async fn run(
        &self,
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
