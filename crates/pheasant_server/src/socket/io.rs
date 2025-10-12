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
use crate::Respond;
use pheasant_core::{ErrorStatus, err_stt};

pub struct ReceiveStream<'a, R: Read> {
    pub stream: &'a mut R,
    buf1: &'a mut Vec<u8>,
}

impl<'a, R: Read> ReceiveStream<'a, R> {
    pub fn new(stream: &'a mut R, buf1: &'a mut Vec<u8>) -> Self {
        Self { buf1, stream }
    }

    pub fn recv(self) -> Result<usize, ErrorStatus> {
        let mut read = 0;
        let mut buf = core::mem::take(self.buf1);
        loop {
            match self.stream.read(&mut buf[read..]) {
                Ok(0) => break,
                Ok(n) => read += n,
                Err(_e) => return err_stt!(?InternalServerError),
                // return SocketError::ReadFailed,
            }
        }
        *self.buf1 = buf;

        Ok(read)
    }
}

pub struct SendStream<'a, W: Write> {
    stream: &'a mut W,
    buf: &'a mut Vec<u8>,
    amount: usize,
}

impl<'a, W: Write> SendStream<'a, W> {
    pub fn new(stream: &'a mut W, buf: &'a mut Vec<u8>, amount: usize) -> Self {
        Self {
            stream,
            buf,
            amount,
        }
    }

    pub fn send(self) -> Result<(), std::io::Error> {
        self.stream.write(&self.buf[..self.amount])?;
        self.stream.flush()?;

        Ok(())
    }
}
