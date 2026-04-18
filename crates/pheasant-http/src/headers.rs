use crate::sidestep_whitespace;
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone)]
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

    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = self.field;
        bytes.extend(b": ");
        bytes.extend(&self.value);
        bytes.push(b'\n');

        bytes
    }
}

// pub fn contains_header(headers: &[Header], header: &[u8]) -> bool {
//     headers.iter().any(|Header { field, .. }| field == header)
// }

// pub fn header_value<'a>(headers: &'a [Header], header: &[u8]) -> Option<&'a [u8]> {
//     headers
//         .iter()
//         .find_map(|Header { field, value }| (field == header).then(|| value.as_slice()))
// }

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

// pub fn headers_into_bytes(headers: Vec<Header>) -> Vec<u8> {
//     headers
//         .into_iter()
//         .map(|h| {
//             let mut hdr = h.field;
//             hdr.extend(b": ");
//             hdr.extend(&h.value);
//             hdr.push(b'\n');

//             hdr
//         })
//         .flatten()
//         .collect()
// }

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Headers(Vec<Header>);

impl From<Vec<Header>> for Headers {
    fn from(v: Vec<Header>) -> Self {
        Self(v)
    }
}

impl core::ops::Deref for Headers {
    type Target = Vec<Header>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Headers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Headers {
    pub fn contains(&self, header: &[u8]) -> bool {
        self.0.iter().any(|Header { field, .. }| field == header)
    }

    pub fn get(&self, header: &[u8]) -> Option<&[u8]> {
        self.0
            .iter()
            .find_map(|Header { field, value }| (field == header).then(|| value.as_slice()))
    }

    /// removes header from self's vec and returns it if it exists
    pub fn remove(&mut self, header: &[u8]) -> Option<Header> {
        let Some(idx) = self
            .0
            .iter()
            .enumerate()
            .find_map(|(i, h)| (h.field_ref() == header).then(|| i))
        else {
            return None;
        };

        Some(self.0.remove(idx))
    }

    pub fn stream_bytes(&self) -> impl IntoIterator<Item = u8> {
        self.0
            .iter()
            .map(|h| [h.field_ref(), b": ", h.value_ref(), b"\n"].concat())
            .flatten()
    }

    pub fn push(&mut self, header: impl Into<Header>) {
        self.0.push(header.into());
    }

    pub fn try_push(&mut self, header: impl TryInto<Header, Error = Error>) -> Result<(), Error> {
        self.0.push(header.try_into()?);

        Ok(())
    }
}

#[derive(Debug)]
pub struct HeadersRef<'a>(&'a [Header]);

impl<'a> From<&'a [Header]> for HeadersRef<'a> {
    fn from(v: &'a [Header]) -> Self {
        Self(v)
    }
}

impl<'a> core::ops::Deref for HeadersRef<'a> {
    type Target = [Header];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> HeadersRef<'a> {
    pub fn contains(&self, header: &[u8]) -> bool {
        self.0.iter().any(|Header { field, .. }| field == header)
    }

    pub fn get(&self, header: &[u8]) -> Option<&[u8]> {
        self.0
            .iter()
            .find_map(|Header { field, value }| (field == header).then(|| value.as_slice()))
    }

    /// removes header from self's vec and returns it if it exists

    pub fn stream_bytes(&self) -> impl IntoIterator<Item = u8> {
        self.0
            .iter()
            .map(|h| [h.field_ref(), b": ", h.value_ref(), b"\n"].concat())
            .flatten()
    }
}

#[derive(Debug)]
pub struct HeadersMut<'a>(&'a mut Vec<Header>);

impl<'a> From<&'a mut Vec<Header>> for HeadersMut<'a> {
    fn from(v: &'a mut Vec<Header>) -> Self {
        Self(v)
    }
}

impl<'a> core::ops::Deref for HeadersMut<'a> {
    type Target = Vec<Header>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> core::ops::DerefMut for HeadersMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> HeadersMut<'a> {
    /// removes header from self's vec and returns it if it exists
    pub fn remove(&mut self, header: &[u8]) -> Option<Header> {
        let Some(idx) = self
            .0
            .iter()
            .enumerate()
            .find_map(|(i, h)| (h.field_ref() == header).then(|| i))
        else {
            return None;
        };

        Some(self.0.remove(idx))
    }

    pub fn push(&mut self, header: impl Into<Header>) {
        self.0.push(header.into());
    }

    pub fn try_push(&mut self, header: impl TryInto<Header, Error = Error>) -> Result<(), Error> {
        self.0.push(header.try_into()?);

        Ok(())
    }
}
