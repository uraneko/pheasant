pub mod core {
    pub use pheasant_core::{
        ClientError, ErrorStatus, Informational, Method, Mime, Protocol, Redirection, ServerError,
        Status, Successful, err_stt, status,
    };
}

pub mod uri {
    pub use pheasant_uri::{Origin, Resource, Route, Url};
}

pub mod server {
    pub use pheasant_server::{Socket, socket};
}
