extern crate std;
use std::io::Write;

use super::Respond;
use crate::Scrutinizer;
use pheasant_core::{ErrorStatus, StatusLiteral, err_stt};
use pheasant_headers::Headers;

impl Respond {
    pub fn parse(self, mut buf: &mut [u8]) -> Result<usize, std::io::Error> {
        println!("{:#?}", self);
        let mut n = buf.write(self.proto.as_bytes())?;
        n += buf.write(&[32])?;

        let code = self.status.code().to_ne_bytes();
        match code.strip_suffix(&[0]) {
            Some(b) => n += buf.write(b)?,
            None => n += buf.write(&code)?,
        }

        n += buf.write(&[32])?;
        n += buf.write(self.status.text().as_bytes())?;
        n += buf.write(&[10])?;
        println!(">>{:?}", &buf[..n + 2]);
        self.headers.write_to(buf)?;
        n += buf.write(&[10])?;
        if let Some(body) = self.body {
            n += buf.write(body.as_slice())?;
        }
        buf.flush()?;

        Ok(n)
    }
}

pub struct ScrutinizeCors<'a> {
    headers: &'a Headers,
    cross_origin: bool,
}

impl<'a> Scrutinizer for ScrutinizeCors<'a> {
    fn scrutinize(&self) -> Result<(), ErrorStatus> {
        if self.cross_origin {
            if !self.headers.contains("Origin")
                || !self.headers.contains("Access-Control-Allow-Methods")
            {
                return err_stt!(?UnprocessableContent);
            }
        }

        Ok(())
    }
}
