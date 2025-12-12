use pheasant_http::{ErrorStatus, Method, request::Request};

pub mod cors;
pub mod errors;
pub mod lookup;
pub mod parse;
pub mod range;
pub mod socket;
pub mod stream;

pub use cors::Cors;
pub use errors::{bad_request, not_found};
pub use parse::parse;
pub use range::Range;
pub use socket::{Socket, bind_socket};
pub use stream::{read_stream, req_buf, write_stream};

pub trait Service<S: Server> {
    async fn run(&self, socket: &mut S, req: Request, buf: &mut Vec<u8>)
    -> Result<(), ErrorStatus>;
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
        buf: &mut Vec<u8>,
        service: impl Service<Self>,
    ) -> Result<(), ErrorStatus>
    where
        Self: Sized,
    {
        service.run(self, req, buf).await
    }

    /// prints out the socket url on the stdout
    fn init_message(&self) {
        println!(
            "\x1b[1;38;2;211;163;104mSocket listening on http://{}\x1b[0m",
            self.addr()
                .map(|addr| addr.to_string())
                .unwrap_or(format!("localhost:{}", self.port())),
        );
    }

    fn addr(&self) -> Result<std::net::SocketAddr, std::io::Error>;

    fn port(&self) -> u16;
}

pub trait Resource<S: Server> {
    fn get(&self, socket: &mut S, req: Request, buf: &mut Vec<u8>) -> Result<(), ErrorStatus> {
        Ok(())
    }

    fn post(&self, socket: &mut S, req: Request, buf: &mut Vec<u8>) -> Result<(), ErrorStatus> {
        Ok(())
    }

    fn put(&self, socket: &mut S, req: Request, buf: &mut Vec<u8>) -> Result<(), ErrorStatus> {
        Ok(())
    }

    fn patch(&self, socket: &mut S, req: Request, buf: &mut Vec<u8>) -> Result<(), ErrorStatus> {
        Ok(())
    }

    fn head(&self, socket: &mut S, req: Request, buf: &mut Vec<u8>) -> Result<(), ErrorStatus> {
        Ok(())
    }

    fn trace(&self, socket: &mut S, req: Request, buf: &mut Vec<u8>) -> Result<(), ErrorStatus> {
        Ok(())
    }

    fn delete(&self, socket: &mut S, req: Request, buf: &mut Vec<u8>) -> Result<(), ErrorStatus> {
        Ok(())
    }

    fn options(&self, socket: &mut S, req: Request, buf: &mut Vec<u8>) -> Result<(), ErrorStatus> {
        Ok(())
    }

    fn connect(&self, socket: &mut S, req: Request, buf: &mut Vec<u8>) -> Result<(), ErrorStatus> {
        Ok(())
    }

    fn run(&self, socket: &mut S, req: Request, buf: &mut Vec<u8>) -> Result<(), ErrorStatus> {
        use Method::*;

        match req.method() {
            Get => self.get(socket, req, buf),
            Post => self.post(socket, req, buf),
            Head => self.head(socket, req, buf),
            Patch => self.patch(socket, req, buf),
            Put => self.put(socket, req, buf),
            Connect => self.connect(socket, req, buf),
            Options => self.options(socket, req, buf),
            Delete => self.delete(socket, req, buf),
            Trace => self.trace(socket, req, buf),
        }
    }
}
