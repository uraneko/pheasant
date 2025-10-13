// lib exports
pub mod core {
    pub use pheasant_core::{
        ClientError, ErrorStatus, Informational, Method, Mime, Protocol, Redirection, ServerError,
        Status, Successful, err_stt, status,
    };
}
// pub use pheasant_macro_utils::RequestOrigin;

pub mod uris {
    pub use pheasant_uri::{Origin, Resource, Route, Url};
}

pub mod server {
    pub use pheasant_server::{Fallback, HttpSocket, Request, Resource, Respond, Servlet};
}

pub mod headers {
    pub use pheasant_headers::{
        Header, Headers, cookies::*, cors::*, message_body_information::*, other::*,
        request_context::*,
    };
}

// macro exports
// pub use pheasant_macro_fail::fail;
// pub use pheasant_macro_get::get;
// pub use pheasant_macro_post::post;
