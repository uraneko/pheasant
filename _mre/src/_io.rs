use embedded_io::{BufRead, ErrorType, Read, ReadExactError, Write};

fn main() {
    read();
    read_exact();
    read_until(5);
    read_until(0);
    read_to(4);
    println!();
}

// should assume that read
// reads everything to the capacity of the buffer that you give
fn read() {
    let mut r: &[u8] = &[1, 2, 3, 4, 5, 6];
    let mut w = [0u8; 4];
    let res = r.read(&mut w);

    println!("---- read ----");
    println!("{:?}", res);
    println!("{:?}", w);
    println!("{:?}", r);
    println!();
}

fn read_exact() {
    let mut r: &[u8] = &[1, 2, 3, 4, 5, 6];
    let mut w = [0u8; 4];
    let res = r.read_exact(&mut w);

    println!("---- exact ----");
    println!("{:?}", res);
    println!("{:?}", w);
    println!("{:?}", r);
    println!();
}

fn read_until(delim: u8) {
    let mut r: &[u8] = &[1, 2, 3, 4, 5, 6];
    let mut w = [0u8; 16];
    let res = r.read_until(&mut w, delim);

    println!("---- until ----");
    println!("{:?}", res);
    println!("{:?}", w);
    println!("{:?}", r);
    println!();
}

fn read_to(to: usize) {
    let mut r: &[u8] = &[1, 2, 3, 4, 5, 6];
    let mut w = [0u8; 4];
    let res = r.read_to(&mut w, to);

    println!("---- to ----");
    println!("{:?}", res);
    println!("{:?}", w);
    println!("{:?}", r);
    println!();
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

    // reads bytes until it finds delim
    // unlike read_until
    // delim not existing in self.inner is an error here
    //
    // if there is an error this function
    // exits without consuming self's inner buffer
    // fn read_with_delim(
    //     &mut self,
    //     delim: u8,
    //     buf: &mut [u8],
    // ) -> Result<usize, ReadExtError<<Self as ErrorType>::Error>> {
    //     let mut read = 0;
    //     while buf.last() != Some(&delim) {
    //         match self.read(buf) {
    //             Ok(0) => return Err(ReadExtError::DelimNotFound),
    //             Ok(n) => read += n,
    //             Err(e) => return Err(ReadExtError::Other(e)),
    //         }
    //     }
    //
    //     Ok(read)
    // }

    /// read from self's buffer up to n bytes
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
