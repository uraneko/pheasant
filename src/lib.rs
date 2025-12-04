pub mod http {
    pub use pheasant_http::{
        ClientError, ErrorStatus, Informational, Method, Mime, Protocol, Redirection, ServerError,
        Status, Successful, err_stt, status,
    };
}

pub mod uri {
    pub use pheasant_uri::{Origin, Resource, Route, Url};
}

pub mod services {
    pub use pheasant_services::{Service, Socket, cors, lookup, parse, read_stream, write_stream};
}
