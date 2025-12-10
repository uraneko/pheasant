use pheasant_http::{Method, request::Request};

pub mod cors;
pub mod errors;
pub mod lookup;
pub mod parse;
pub mod socket;
pub mod stream;

pub use cors::Cors;
pub use errors::{bad_request, not_found};
// pub use lookup::lookup;
pub use parse::parse;
pub use socket::Socket;
pub use stream::{read_stream, req_buf, write_stream};

pub trait Service<S: Server> {
    fn run(&self, socket: &mut S, req: Request, buf: &mut String);
}

pub trait Server {
    async fn event_loop(&mut self, fun: impl AsyncFn(&mut Self));

    // core services are run through the socket
    // this would ofc mean that the socket stores the server state
    // or whatever state is needed by the services to run correctly
    async fn service(&mut self, req: Request, buf: &mut String, service: impl Service<Self>)
    where
        Self: Sized;

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

pub trait Resource {
    fn get(&self, req: Request, buf: &mut String) {}

    fn post(&self, req: Request, buf: &mut String) {}

    fn put(&self, req: Request, buf: &mut String) {}

    fn patch(&self, req: Request, buf: &mut String) {}

    fn head(&self, req: Request, buf: &mut String) {}

    fn trace(&self, req: Request, buf: &mut String) {}

    fn delete(&self, req: Request, buf: &mut String) {}

    fn options(&self, req: Request, buf: &mut String) {}

    fn connect(&self, req: Request, buf: &mut String) {}

    fn run(&self, req: Request, buf: &mut String) {
        use Method::*;

        match req.method() {
            Get => self.get(req, buf),
            Post => self.post(req, buf),
            Head => self.head(req, buf),
            Patch => self.patch(req, buf),
            Put => self.put(req, buf),
            Connect => self.connect(req, buf),
            Options => self.options(req, buf),
            Delete => self.delete(req, buf),
            Trace => self.trace(req, buf),
        }
    }
}
