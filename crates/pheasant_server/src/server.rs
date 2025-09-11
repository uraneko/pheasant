use hashbrown::HashSet;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};

use super::{
    ClientError, Fallback, HttpSocket, Method, PheasantError, PheasantResult, Process,
    ProcessBundle, Protocol, Redirection, Request, Response, ResponseStatus, Route, ServerError,
    Status, Successful,
};

pub trait Server {
    /// start the server
    /// this means that socket starts listening
    /// and uses the EventLoop implementation to handle incoming connections
    fn start(&mut self);

    /// shuts down the server instance
    fn terminate(mut self);

    /// puts the server main thread to sleep
    fn sleep(&mut self);
}

impl Server for Socket {}

// WARN when responding to a credentialed request, the CORS glob/* header value is not allowed for the following headers
// Access-Control-Allow-Origin, Access-Control-Allow-Headers, Access-Control-Allow-Methods and Access-Control-Expose-Headers
// TODO Server.origins { whitelist, blacklist }
