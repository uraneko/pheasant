trait Requester: Read {
    fn read_req(&mut self) -> Result<Request, ()>;
}

trait Respondent: Write {
    fn write_res(&mut self, resp: Respond) -> Result<(), ()>;

    fn write_err(&mut self, err: HttpError) -> Result<(), ()>;

    fn write_opt(&mut self, opt: Preflight) -> Result<(), ()>;

    fn write_frw(&mut self, frw: Forward) -> Result<(), ()>;
}

enum ReadUntilError<E> {
    DelimNotFound,
    Other(E),
}

// extended io methods for the embedded_io
pub trait IOExt: Read {
    // reads bytes until it finds delim
    // unlike read_until
    // delim not existing in self.inner is an error here
    fn read_with_delim(&mut self, delim: u8, buf: &mut [u8]) -> Result<usize, ReadUntilError> {
        let mut read = 0;
        while buf.last() != Some(&delim) {
            match self.read(buf) {
                Ok(0) => return Err(ReadUntilError::DelimNotFound),
                Ok(n) => read += n,
                Err(e) => return Err(ReadUntilError::Other(e)),
            }
        }

        self.consume(read);
        Ok(read)
    }

    fn read_until(&mut self, delim: u8, buf: &mut [u8]) -> Result<usize, ReadUntilError> {
        let mut read = 0;
        while buf.last() != Some(&delim) {
            match self.read(buf) {
                Ok(n) => read += n,
                Err(e) => return Err(ReadUntilError::Other(e)),
            }
        }

        Ok(read)
    }
}
