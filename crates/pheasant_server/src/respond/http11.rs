extern crate std;
use std::io::{BufWriter, Write};

use super::Respond;
use crate::Scrutinizer;
use pheasant_core::{ErrorStatus, StatusLiterals, err_stt};
use pheasant_headers::Headers;

impl Respond {
    pub fn parse(self, mut buf: BufWriter<&mut impl Write>) -> Result<usize, std::io::Error> {
        let mut n = buf.write(self.proto.as_bytes())?;
        n += buf.write(&[32])?;

        let code = self.status.code().to_string();
        n += buf.write(code.as_bytes())?;
        n += buf.write(&[32])?;
        n += buf.write(self.status.text().as_bytes())?;
        n += buf.write(&[10])?;
        self.headers.write_to(&mut buf)?;
        n += buf.write(&[10])?;
        if let Some(body) = self.body {
            // NOTE body is not sure to be valid utf8
            // since it could have gone through some sort of encoding
            n += buf.write(body.as_slice())?;
        }
        buf.flush()?;

        Ok(n)
    }

    pub fn scrutinize(&self, is_cross_origin: bool) -> Result<(), ErrorStatus> {
        ScrutinizeCors::new(&self.headers, is_cross_origin).scrutinize()?;

        Ok(())
    }
}

pub struct ScrutinizeCors<'a> {
    headers: &'a Headers,
    cross_origin: bool,
}

impl<'a> ScrutinizeCors<'a> {
    fn new(headers: &'a Headers, cross_origin: bool) -> Self {
        Self {
            headers,
            cross_origin,
        }
    }
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
