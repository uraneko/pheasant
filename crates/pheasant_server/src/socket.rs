use pheasant_core::Protocol;
use std::io::{BufRead, BufReader, Result as IoRes, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener};

pub mod builder;

// M is a resources middlewares enum
// S is a server middlewares enum
pub struct Socket {
    socket: TcpListener,
    buffer: Vec<u8>,
}

impl Socket {
    /// # Example
    /// use like so
    /// ```
    /// let server =
    ///    Socket::builder("127.0.0.1:7859")?
    ///      .service(SomeService::new(...))
    ///      .wrapper(SomeWrapper::new(...))
    ///      .build();
    /// ```
    ///
    /// # Error
    /// returns an Err if the Tcp Listener generation fails
    pub fn builder(addr: impl Into<Ipv4Addr>, port: u16) -> Result<builder::Builder, Error> {
        let socket = bind_socket(addr, port)?;

        Ok(builder::Builder::new(socket))
    }

    // returns a result of the socket's ip addr
    fn addr(&self) -> IoRes<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn port(&self) -> u16 {
        match self.socket.local_addr() {
            Ok(addr) => addr.port(),
            Err(_) => 80,
        }
    }

    /// prints out the socket url on the stdout
    pub fn init_message(&self) {
        println!(
            "\x1b[1;38;2;111;163;204mSocket listening on http://localhost:{}\x1b[0m",
            self.port()
        );
    }
}

impl Socket {
    pub async fn event_loop(&mut self) {
        let mut buf = String::new();
        while let Ok((mut stream, _)) = self.socket.accept() {
            let mut reader = BufReader::new(&mut stream);
            buf.clear();
            loop {
                _ = reader.read_line(&mut buf);
                println!("<{}>", buf);
                if buf.ends_with("\r\n\r\n") {
                    break;
                }
            }
            _ = stream
                .write(b"HTTP/1.1 200 Ok\nContent-Type:text/htmlContent-Length: 9\n\nhello web");
            _ = stream.flush();
        }
    }
}

/// tries to bind the socket to the passed addr and port
/// keeps incrementing port number until it finds a free port
///
/// ### Error
/// - returns an std::io::Error when port reaches u16::MAX and no free port is found
pub fn bind_socket(addr: impl Into<Ipv4Addr>, mut port: u16) -> Result<TcpListener, Error> {
    let addr = addr.into();
    let socket = loop {
        match TcpListener::bind((addr, port)) {
            Ok(listener) => break listener,
            err if port == u16::MAX => return err.map_err(|_| Error::None),
            _err => port += 1,
        }
    };

    Ok(socket)
}

// converts a slice of Protocols to a u8
pub fn proto_slice_to_u8(protos: &[Protocol]) -> u8 {
    use Protocol::*;

    let mut byte = 0;
    if protos.contains(&Http11) {
        byte |= 1;
    }
    if protos.contains(&Http2) {
        byte |= 2;
    }

    byte
}

#[derive(Debug)]
pub enum Error {
    None,
}
