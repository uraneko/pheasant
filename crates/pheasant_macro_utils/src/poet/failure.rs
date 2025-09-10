use proc_macro2::{Span, TokenStream as TS2};
use quote::quote;
use std::collections::HashSet;
use syn::{FnArg, Ident, ItemFn, PatType, Type, Visibility};

use super::Altering;
use crate::{FallbackPlumber, Mining};
use pheasant_core::{Cors, Method, Mime};
use pheasant_uri::Route;

#[derive(Debug)]
pub struct FallbackPoet {
    fun: ItemFn,
    status: u16,
    mime: Option<Mime>,
    decorated: bool,
}

impl FallbackPoet {
    pub fn new(mut plumber: FallbackPlumber) -> Self {
        let status = plumber.status();
        let mime = plumber.take_mime();
        let fun = plumber.into_fun();
        let decorated = fun.is_decorated();

        FallbackPoet {
            status,
            fun,
            mime,
            decorated,
        }
    }
}

pub trait FallbackInscriptions {
    fn mime(&self) -> TS2;

    fn status(&self) -> TS2;

    fn assemble_decorator_fun(&self) -> Option<TS2>;

    fn assemble_bundler_fun(&self) -> TS2;

    fn assemble(&mut self) -> TS2;
}

impl FallbackInscriptions for FallbackPoet {
    fn mime(&self) -> TS2 {
        if let Some(ref mime) = self.mime {
            let mime = mime.essence_str();
            quote! {
                Some(pheasant::Mime::macro_checked(#mime))
            }
        } else {
            quote! { None }
        }
    }

    fn status(&self) -> TS2 {
        let status = pheasant_core::ErrorStatus::try_from(self.status).unwrap();

        quote! { pheasant:: #status }
    }

    fn assemble_decorator_fun(&self) -> Option<TS2> {
        let fun = &self.fun;
        let vis = fun.vis();
        let failure = fun.decorate_ident("_failure");
        let decorated = fun.decorate_ident("_decorator");
        let status = self.status();

        (!self.decorated).then(|| {
            quote! {
            #vis async fn #decorated() -> pheasant::Response {
                let mut resp = pheasant::Response::failing(#status);
                let data = #failure().await;
                resp.update_body(data);

                resp
            }}
        })
    }

    fn assemble_bundler_fun(&self) -> TS2 {
        let fun = &self.fun;
        let vis = fun.vis();
        let mime = self.mime();
        let status = self.status;

        let bundler = &fun.sig.ident;
        let maybe_decorated = if self.decorated {
            self.fun.decorate_ident("_failure")
        } else {
            fun.decorate_ident("_decorator")
        };

        quote! {
            #vis fn #bundler() -> pheasant::Fallback {
                pheasant::Fallback::new(#status, #mime, #maybe_decorated)
            }
        }
    }

    fn assemble(&mut self) -> TS2 {
        let bundler_fun = self.assemble_bundler_fun();
        let decorated_fun = self.assemble_decorator_fun();
        self.fun.sig.ident = self.fun.decorate_ident("_failure");

        let fun = &self.fun;

        quote! {
            #fun

            #decorated_fun

            #bundler_fun
        }
    }
}
