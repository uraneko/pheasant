use crate::{Method, Protocol, Status, status};
use alloc::vec::Vec;
use std::io::Write;

#[derive(Debug)]
pub struct Respond {
    proto: Protocol,
    status: Status,
    headers: Vec<u8>,
    body: Vec<u8>,
}

impl Respond {
    pub fn new(proto: Protocol, status: Status) -> Self {
        Self {
            proto,
            status,
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn proto(&mut self, proto: Protocol) -> &mut Self {
        self.proto = proto;

        self
    }

    pub fn proto_cpy(&self) -> Protocol {
        self.proto
    }

    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = status;

        self
    }

    pub fn status_cpy(&self) -> Status {
        self.status
    }

    pub fn headers_mut(&mut self) -> &mut Vec<u8> {
        &mut self.headers
    }

    pub fn headers_ref(&self) -> &[u8] {
        &self.headers
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }

    pub fn body_ref(&self) -> &[u8] {
        &self.body
    }

    pub fn to_bytes(&self, method: Method) -> Vec<u8> {
        let mut payload = [
            self.proto.as_bytes(),
            &[32],
            self.status.as_bytes(),
            &[10],
            &self.headers,
            &[10],
        ]
        .concat();

        if ![Method::Connect, Method::Head].contains(&method) && !self.body.is_empty() {
            payload.extend(&self.body);
        }

        payload
    }

    // this doesnt clear
    // user can do so on their own
    /// writes self as bytes to the passed buffer
    pub fn dump_bytes(&self, buf: &mut Vec<u8>, method: Method) {
        buf.extend(&self.to_bytes(method));
    }

    /// writes the response bytes directly to a tcp stream
    pub fn direct_write(
        &self,
        writer: &mut impl Write,
        method: Method,
    ) -> Result<(), std::io::Error> {
        writer.write_all(&self.to_bytes(method))?;
        writer.flush()?;

        Ok(())
    }

    // resets proto and status to defaults
    // and clears headers and body
    pub fn clear(&mut self) {
        self.proto = Protocol::Http11;
        self.status = status!(200);
        self.headers.clear();
        self.body.clear();
    }
}
