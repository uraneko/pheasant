use crate::{ErrorMessage, Forward, Preflight, Request, Respond};
use embedded_io::{BufRead, ErrorType, Read, ReadExactError, Write};
use pheasant_core::ErrorStatus;

pub trait Requester: Read {
    fn read_req(&mut self) -> Result<Request, ()>;
}

trait Respondent: Write {
    fn write_res(&mut self, resp: Respond) -> Result<(), ()>;

    fn write_err(&mut self, err: ErrorMessage<ErrorStatus>) -> Result<(), ()>;

    fn write_opt(&mut self, opt: Preflight) -> Result<(), ()>;

    fn write_frw(&mut self, frw: Forward) -> Result<(), ()>;
}

#[derive(Debug)]
pub enum ReadExtError<E> {
    DelimNotFound,
    BufTooSmall,
    Other(E),
}

// extended io methods for the embedded_io
pub trait ReadExt: Read + BufRead {
    // if delim in self.inner this reads until delim
    // else reads to the end
    fn read_until(
        &mut self,
        buf: &mut [u8],
        delim: u8,
    ) -> Result<usize, ReadExtError<ReadExactError<<Self as ErrorType>::Error>>>;

    /// read from self's buffer up to n bytes
    /// this is the same as using read_exact[..range]
    /// except this returns an error if range is out of bounds
    fn read_to(
        &mut self,
        buf: &mut [u8],
        range: usize,
    ) -> Result<(), ReadExtError<<Self as ErrorType>::Error>> {
        if buf.len() < range {
            return Err(ReadExtError::BufTooSmall);
        }
        self.read_exact(&mut buf[..range]).unwrap();

        Ok(())
    }
}

impl ReadExt for &[u8] {
    fn read_until(
        &mut self,
        mut buf: &mut [u8],
        delim: u8,
    ) -> Result<usize, ReadExtError<ReadExactError<<Self as ErrorType>::Error>>> {
        let read = match self.iter().position(|b| b == &delim) {
            Some(idx) => {
                if buf.len() < idx {
                    return Err(ReadExtError::BufTooSmall);
                }
                self.read_exact(&mut buf[..idx]).unwrap();

                idx
            }
            None => {
                let n = core::cmp::min(buf.len(), self.len());
                self.read(&mut buf).unwrap();

                n
            }
        };

        Ok(read)
    }
}
