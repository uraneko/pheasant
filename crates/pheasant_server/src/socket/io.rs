//! cant use embedded_io until no_std ip/tcp is implemented
//! until then will be using std's Read and TcpStream

//! the request is processed once it's raw data is received through a client connection
//!
//! the prerequisite to respond needs 2 inputs: request + resource
//! the condition is
//! req.method == res.method && req.route == res.route -> Respond
//!
//! the prerequisite for a forward is that the response condition fails at route matching
//! the condition is
//! there exists a resource such that res.allows_method(req.method) &&
//! res.redirects.contains(req.route)
//!
//! the prerequisite for a preflight is that req.method == Options
//! the condition is
//! referring to req.requested_method as m; there exists a resource such that res.m is registered
//! and allows cors requests
//!
//! the prerequisite to negotiate is that the request includes the Expect or the Upgrade&Connection headers
//! the condition is
//! 101 -
//! req.headers.contains(Upgrade + Connection) && the server decides to follow through with the
//! upgrade -> we respond with a 101 switching protos
//! 100 -
//! req.headers.contains(Expect = 100-Continue) -> server returns that status code iif it
//! decides to keep the first part of the request and process it
//! 102 -
//! 102 status is deprecated
//! 103 -
//! rarely supported on proto < http2
//! server sends 103 with a Link header to tell the client to preload a resource before the server
//! sends its actual response
//!
//! the prerequisite for an error is that any of the preceeding message variants (req/res/frd/prf)
//! errors out at any point before responding to the client
//! the condition is nothing

extern crate std;
use std::io::{Read, Write};
use std::{
    io::{BufReader, BufWriter, Result as IoRes},
    net::TcpStream,
};

// use super::SocketError;
use crate::request::http11::lex;
use crate::{Request, Respond};
use pheasant_core::{ErrorStatus, err_stt};

#[derive(Debug)]
pub enum ReadUntilError {
    DelimNotFound,
    BufTooSmall,
    // Other(E),
}

// TODO account for read not reading all bytes for sure
// extended io methods for the embedded_io
pub trait ReadUntil: Read {
    // if delim in self.inner this reads until delim
    // else reads to the end
    fn read_until(&mut self, buf: &mut [u8], delim: u8) -> Result<usize, ReadUntilError>;
}

impl ReadUntil for &[u8] {
    fn read_until(&mut self, mut buf: &mut [u8], delim: u8) -> Result<usize, ReadUntilError> {
        let mut read = match self.iter().position(|b| b == &delim) {
            Some(idx) => {
                if buf.len() < idx {
                    return Err(ReadUntilError::BufTooSmall);
                }
                self.read_exact(&mut buf[..idx + 1]).unwrap();
                buf[idx] = 0;

                idx
            }
            None => {
                let n = core::cmp::min(buf.len(), self.len());

                while let Ok(r) = self.read(&mut buf) {
                    if r == 0 {
                        break;
                    }
                }

                n
            }
        };

        if read > 1 && buf[read - 1] == 13 {
            buf[read - 1] = 0;

            read -= 1;
        }

        Ok(read)
    }
}

impl<'a, R: Read> Read for ReceiveStream<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> IoRes<usize> {
        self.stream.read(buf)
    }
}

// impl<'a, R: Read> Read for ReceiveStream<'a, R> {
//     fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
//         self.stream.fill_buf()
//     }
//
//     fn consume(&mut self, amount: usize) {
//         self.stream.consume(amount)
//     }
// }
//
// impl<'a, W: Write> Write for SendStream<'a, W> {
//     fn write(&mut self, buf: &[u8]) -> IoRes<usize> {
//         self.stream.write(buf)
//     }
//
//     fn flush(&mut self) -> IoRes<()> {
//         self.stream.flush()
//     }
// }

pub struct ReceiveStream<'a, R: Read> {
    pub stream: &'a mut R,
    buf: &'a mut [u8],
}

impl<'a, R: Read> ReceiveStream<'a, R> {
    pub fn new(buf: &'a mut [u8], stream: &'a mut R) -> Self {
        Self { buf, stream }
    }

    pub fn recv(mut self) -> Result<Request, ErrorStatus> {
        let mut read = 0;
        let buf = core::mem::take(&mut self.buf);
        loop {
            match self.read(&mut buf[read..]) {
                Ok(0) => break,
                Ok(n) => read += n,
                Err(_e) => return err_stt!(?InternalServerError),
                // return SocketError::ReadFailed,
            }
        }
        self.buf = buf;

        let mut temp = [0; 1024];
        let tokens = lex(&self.buf[..read], &mut temp);
        if tokens.is_empty() {
            return err_stt!(?BadRequest);
        }

        Request::parse(tokens)
    }
}

pub struct SendStream<'a, W: Write> {
    stream: &'a mut W,
    buf: &'a mut [u8],
    res: Respond,
}

impl<'a, W: Write> SendStream<'a, W> {
    pub fn new(stream: &'a mut W, buf: &'a mut [u8], res: Respond) -> Self {
        Self { res, buf, stream }
    }

    pub fn send(self) -> Result<(), std::io::Error> {
        let n = self.res.parse(self.buf)?;
        println!("{:?}", &self.buf[..n]);
        self.stream.write(&self.buf[..n])?;
        self.stream.flush()?;

        Ok(())
    }
}
