// lib exports
pub use pheasant_core::{
    ClientError, Cookie, Cors, ErrorStatus, Fallback, Header, HeaderMap, Informational, Method,
    Mime, Process, ProcessBundle, Protocol, Redirection, Request, Response, Server, ServerError,
    Status, Successful,
};
// pub use pheasant_macro_utils::RequestOrigin;
pub use pheasant_uri::{Origin, OriginSet, Resource, Route, Url};

// macro exports
// pub use pheasant_macro_fail::fail;
// pub use pheasant_macro_get::get;
// pub use pheasant_macro_post::post;
