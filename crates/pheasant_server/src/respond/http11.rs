extern crate std;
use std::io::Write;

use super::Respond;
use pheasant_core::StatusLiteral;

impl Respond {
    pub fn parse(self, mut buf: &mut [u8]) -> Result<(), std::io::Error> {
        buf.write(self.proto.as_bytes())?;
        buf.write(&self.status.code().to_ne_bytes())?;
        buf.write(self.status.text().as_bytes())?;
        buf.write(&[10])?;
        self.headers.write_to(buf)?;
        buf.write(&[10])?;
        if let Some(body) = self.body {
            buf.write(body.as_slice())?;
        }
        buf.flush()?;

        Ok(())
    }
}
