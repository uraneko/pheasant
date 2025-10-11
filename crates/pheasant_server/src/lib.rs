extern crate std;
use hashbrown::HashSet;
use pheasant_core::ErrorStatus;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};

pub mod fallback;
pub mod request;
pub mod resource;
pub mod respond;
pub mod servlet;
pub mod socket;

pub use fallback::Fallback;
pub use request::Request;
pub use resource::Resource;
pub use respond::Respond;
pub use servlet::{Servlet, ServletBundle};
pub use socket::{HttpSocket, SocketRef};

// TODO Respond Forward Preflight and HttpError logic
// TODO Scrutinizer impls

/// data types that can be sent in http message
pub trait OctetStream {
    fn octet_stream(&self) -> &[u8];
}

/// validates that the read request's various parts are valid
/// e.g., Pragma: ... header + Http1.1 protocol is an error
///
/// scrutinize a request's contents
pub trait Scrutinizer {
    fn scrutinize(&self) -> Result<(), ErrorStatus>;
}

pub trait Server {
    /// start the server
    /// this means the socket(s) start(s) listening
    /// and uses the EventLoop implementation to handle incoming connections
    fn start(&mut self);

    /// shuts down the server instance
    fn terminate(self);

    /// puts the server main thread to sleep
    fn sleep(&mut self);
}

// impl Server for Socket {}

// WARN when responding to a credentialed request, the CORS glob/* header value is not allowed for the following headers
// Access-Control-Allow-Origin, Access-Control-Allow-Headers, Access-Control-Allow-Methods and Access-Control-Expose-Headers

pub trait EventLoop {
    /// handles the socket to socket communication
    fn message(&mut self);

    /// contains the logic for the repeating event loop
    /// usually should contain a while, for or loop block
    fn event_loop(&mut self);
}

// impl EventLoop for Socket {}
