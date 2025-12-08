use pheasant_http::request::Request;

pub mod cors;
pub mod errors;
pub mod lookup;
pub mod parse;
pub mod socket;
pub mod stream;

pub use cors::cors;
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
    fn get() {}
    fn post() {}
    fn put() {}
    fn patch() {}
    fn head() {}
    fn connect() {}
    fn delete() {}
    fn options() {}
}
