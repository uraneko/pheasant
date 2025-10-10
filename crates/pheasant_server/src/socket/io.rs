//! cant use embedded_io until no_std ip/tcp is implemented
//! until then will be using std's Read and TcpStream
extern crate std;
use std::io::{BufRead, Write};
use std::{
    io::{BufReader, BufWriter, Read, Result as IoRes},
    net::TcpStream,
};

use crate::request::http11::lex;
use crate::{Request, Respond};
use pheasant_core::{ErrorStatus, err_stt};

impl<'a, R: BufRead> ReceiveStream<'a, R> {
    fn read_req(&mut self) -> Result<Request, ErrorStatus> {
        let mut read = 0;
        let mut buf = core::mem::take(&mut self.buf);
        loop {
            match self.read(&mut buf[read..]) {
                Ok(0) => break,
                Ok(n) => read += n,
                Err(_e) => return err_stt!(?InternalServerError),
            }
        }
        self.buf = buf;

        let tokens = lex(&self.buf[..read], &mut []);
        if tokens.is_empty() {
            return err_stt!(?BadRequest);
        }
        self.consume(read);

        Request::parse(tokens)
    }
}

#[derive(Debug)]
pub enum ReadUntilError {
    DelimNotFound,
    BufTooSmall,
    // Other(E),
}

// TODO account for read not reading all bytes for sure
// extended io methods for the embedded_io
pub trait ReadUntil: BufRead {
    // if delim in self.inner this reads until delim
    // else reads to the end
    fn read_until(&mut self, buf: &mut [u8], delim: u8) -> Result<usize, ReadUntilError>;
}

impl ReadUntil for &[u8] {
    fn read_until(&mut self, mut buf: &mut [u8], delim: u8) -> Result<usize, ReadUntilError> {
        let read = match self.iter().position(|b| b == &delim) {
            Some(idx) => {
                if buf.len() < idx {
                    return Err(ReadUntilError::BufTooSmall);
                }
                self.read_exact(&mut buf[..idx]).unwrap();

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

        Ok(read)
    }
}

pub type TcpBufR = BufReader<TcpStream>;
pub type TcpBufW = BufWriter<TcpStream>;

impl<'a, R: Read> Read for ReceiveStream<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> IoRes<usize> {
        self.stream.read(buf)
    }
}

impl<'a, R: BufRead> BufRead for ReceiveStream<'a, R> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.stream.fill_buf()
    }

    fn consume(&mut self, amount: usize) {
        self.stream.consume(amount)
    }
}

impl<'a, W: Write> Write for SendStream<'a, W> {
    fn write(&mut self, buf: &[u8]) -> IoRes<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> IoRes<()> {
        self.stream.flush()
    }
}

pub struct ReceiveStream<'a, R: Read> {
    pub stream: &'a mut R,
    buf: &'a mut [u8],
}

impl<'a, R: Read> ReceiveStream<'a, R> {
    pub fn new(buf: &'a mut [u8], stream: &'a mut R) -> Self {
        Self { buf, stream }
    }

    pub fn recv(self) -> Result<Request, std::io::Error> {
        todo!()
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
        self.res.parse(self.buf)?;
        self.stream.write_all(self.buf)?;
        self.stream.flush()?;

        Ok(())
    }
}
