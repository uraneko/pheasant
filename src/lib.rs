pub mod http {
    pub use pheasant_http::{
        ClientError, ErrorStatus, Informational, Method, Mime, Protocol, Redirection, ServerError,
        Status, Successful, err_stt, request, status,
    };
}

pub mod uri {
    pub use pheasant_uri::{Origin, Query, Resource, Route, Url};
}

pub mod services {
    pub use pheasant_services::{
        Cors, Range, Resource, Server, Service, Socket, bad_request, bind_socket, cors, not_found,
        parse, read_stream, req_buf, write_stream,
    };
}
