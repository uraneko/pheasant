#![no_std]
#[allow(refining_impl_trait)]
extern crate alloc;
use alloc::string::String;
use hashbrown::{HashMap, HashSet};
use pheasant_core::ErrorStatus;

pub mod headers;
pub use headers::*;

pub trait MessageHeadersMap {
    fn pull(&mut self, h: &str) -> Option<String>;

    fn pull_iter<P>(
        &mut self,
        p: P,
    ) -> impl Iterator<Item = (impl Into<String>, impl Into<String>)>
    where
        P: FnMut(&String, &mut String) -> bool;

    fn push(&mut self, h: &str, th: impl ToHeader);

    fn push_iter(&mut self, th: impl ToHeaders);
}

pub trait MessageHeadersSet {
    fn push(&mut self, th: impl ToHeader);

    fn push_iter(&mut self, th: impl ToHeaders);
}

impl MessageHeadersSet for HashSet<String> {
    fn push(&mut self, th: impl ToHeader) {
        self.insert(th.to_header().into());
    }

    fn push_iter(&mut self, th: impl ToHeaders) {
        self.extend(th.to_headers().map(|(a, b)| a.into()));
    }
}

impl MessageHeadersMap for HashMap<String, String> {
    fn pull(&mut self, h: &str) -> Option<String> {
        self.remove(h)
    }

    fn pull_iter<P>(&mut self, p: P) -> impl Iterator<Item = (impl Into<String>, impl Into<String>)>
    where
        P: FnMut(&String, &mut String) -> bool,
    {
        self.extract_if(p)
    }

    fn push(&mut self, h: &str, th: impl ToHeader) {
        self.insert(h.into(), th.to_header().into());
    }

    fn push_iter(&mut self, th: impl ToHeaders) {
        self.extend(th.to_headers().map(|(k, v)| (k.into(), v.into())));
    }
}

pub type HttpResult<T> = Result<T, ErrorStatus>;

pub trait FromHeaders<'a>
where
    Self: Sized,
{
    type Headers;
    fn from_headers(h: Self::Headers) -> HttpResult<Self>;
}

pub trait FromHeader
where
    Self: Sized,
{
    fn from_header(header: String) -> HttpResult<Self>;
}

pub trait ToHeaders {
    fn to_headers(&self) -> impl Iterator<Item = (impl Into<String>, impl Into<String>)>;
}

pub trait ToHeader {
    fn to_header(&self) -> impl Into<String>;
}
