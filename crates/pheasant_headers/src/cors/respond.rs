extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use hashbrown::HashSet;

use super::{CorsConfigs, CorsHeader, RequestCors};
use chrono::TimeDelta;
use pheasant_core::{ErrorStatus, Header, Headers, MaybeGlob, Method, err_stt};
use pheasant_uri::Origin;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RespondCors<'a> {
    methods: Vec<Method>,
    headers: &'a HashSet<CorsHeader>,
    origin: MaybeGlob<&'a Origin>,
    credentials: bool,
    max_age: Option<TimeDelta>,
}

impl<'a> RespondCors<'a> {
    pub fn new(
        configs: &'a CorsConfigs,
        request: &'a RequestCors,
        resource_methods: Vec<Method>,
    ) -> Result<Self, ErrorStatus> {
        if !configs.allows_origin(request.origin_ref()) {
            return err_stt!(?Forbidden);
        }

        Ok(Self {
            methods: resource_methods,
            max_age: configs.max_age,
            credentials: configs.credentials,
            origin: request.origin_ref(),
            headers: &configs.headers,
        })
    }
}

impl<'a> From<RespondCors<'a>> for Headers {
    fn from(cors: RespondCors<'a>) -> Headers {
        Some((
            "Access-Control-Allow-Origin".to_owned(),
            Header::Field(cors.origin.into()),
        ))
        .into_iter()
        .chain(Some((
            "Access-Control-Allow-Methods".to_owned(),
            Header::Field(chain_methods(&cors.methods)),
        )))
        .chain(Some((
            "Access-Control-Allow-Headers".to_owned(),
            Header::Field(chain_headers(cors.headers)),
        )))
        .chain(chain_exposable(cors.headers).map(|ex| {
            (
                "Access-Control-Expose-Headers".to_owned(),
                Header::Field(ex),
            )
        }))
        .chain(cors.credentials.then(|| {
            (
                "Access-Control-Allow-Credintials".to_owned(),
                Header::Field("true".to_owned()),
            )
        }))
        .chain(cors.max_age.map(|ma| {
            (
                "Access-Control-Max-Age".to_owned(),
                Header::Field(ma.to_string()),
            )
        }))
        .collect()
    }
}

fn chain_methods(methods: &[Method]) -> String {
    let mut m = methods
        .into_iter()
        .fold("".to_owned(), |acc, m| acc + m.as_str() + ", ");

    m.pop();
    m.pop();

    m
}

fn chain_headers(headers: &HashSet<CorsHeader>) -> String {
    let mut h = headers
        .into_iter()
        .map(|h| &h.header)
        .fold("".to_owned(), |acc, h| acc + h + ", ");
    h.pop();
    h.pop();

    h
}

fn chain_exposable(headers: &HashSet<CorsHeader>) -> Option<String> {
    headers.iter().any(|h| h.expose).then(|| {
        let mut h = headers
            .into_iter()
            .filter(|h| h.expose)
            .map(|h| &h.header)
            .fold("".to_owned(), |acc, h| acc + h + ", ");

        h.pop();
        h.pop();

        h
    })
}
