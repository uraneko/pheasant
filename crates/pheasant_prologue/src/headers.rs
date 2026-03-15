use crate::sidestep_whitespace;
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Clone)]
pub struct Header {
    field: Vec<u8>,
    value: Vec<u8>,
}

impl Header {
    pub fn new(field: Vec<u8>, value: Vec<u8>) -> Self {
        Self { field, value }
    }

    pub fn field_ref(&self) -> &[u8] {
        &self.field
    }

    pub fn value_ref(&self) -> &[u8] {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut Vec<u8> {
        &mut self.value
    }

    pub fn push(&mut self, b: u8) {
        self.value.push(b);
    }

    pub fn extend(&mut self, slice: &[u8]) {
        self.value.extend(slice);
    }
}

pub fn contains_header(headers: &[Header], header: &[u8]) -> bool {
    headers.iter().any(|Header { field, .. }| field == header)
}

pub fn header_value<'a>(headers: &'a [Header], header: &[u8]) -> Option<&'a [u8]> {
    headers
        .iter()
        .find_map(|Header { field, value }| (field == header).then(|| value.as_slice()))
}

pub enum Error {
    UnparsableHeaderBytes,
}

impl<'a> TryFrom<&'a [u8]> for Header {
    type Error = Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let Some(colon) = slice.iter().position(|b| *b == b':') else {
            return Err(Error::UnparsableHeaderBytes);
        };

        // whitespace
        let field = slice[..colon].to_vec();
        let start = sidestep_whitespace(slice, colon + 1);
        let value = slice[start..].to_vec();

        Ok(Self { field, value })
    }
}

fn unwhitespace_slice(value: &[u8]) -> Vec<u8> {
    let start = sidestep_whitespace(value, 0);

    value[start..].to_vec()
}

impl<'a> From<(&'a [u8], &'a [u8])> for Header {
    fn from(slices: (&'a [u8], &'a [u8])) -> Self {
        Self {
            field: slices.0.to_vec(),
            value: unwhitespace_slice(slices.1),
        }
    }
}

impl<'a> From<[&'a [u8]; 2]> for Header {
    fn from(slices: [&[u8]; 2]) -> Self {
        Self {
            field: slices[0].to_vec(),
            value: unwhitespace_slice(slices[1]),
        }
    }
}

fn unwhitespace_vec(mut value: Vec<u8>) -> Vec<u8> {
    let start = sidestep_whitespace(&value, 0);
    if start > 0 {
        _ = value.drain(..start - 1);
    }

    value
}

impl From<(Vec<u8>, Vec<u8>)> for Header {
    fn from(vecs: (Vec<u8>, Vec<u8>)) -> Self {
        Self {
            field: vecs.0,
            value: unwhitespace_vec(vecs.1),
        }
    }
}

impl From<[Vec<u8>; 2]> for Header {
    fn from(mut vecs: [Vec<u8>; 2]) -> Self {
        Self {
            field: core::mem::take(&mut vecs[0]),
            value: unwhitespace_vec(core::mem::take(&mut vecs[1])),
        }
    }
}
