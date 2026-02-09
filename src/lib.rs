pub mod http {
    pub use pheasant_http::{
        ClientError, ErrorStatus, Header, Informational, Method, Mime, Protocol, Redirection,
        ServerError, Status, Successful, client, contains_header, err_stt, header_value, server,
        status,
    };
}

pub mod uri {
    pub use pheasant_uri::{Origin, Query, Resource, Url};
}

pub mod services {
    pub use pheasant_services::{
        Cors, Forward, MessageBodyInfo, Ranges, ReadCookies, Resource, Server, Service, Socket,
        WriteCookies, bind_socket, cors, date, http_error, parse, print, read_stream, req_buf,
        resp_write_stream, support_ranges, write_stream,
    };
}
