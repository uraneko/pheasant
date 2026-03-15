pub mod prologue {
    pub use pheasant_prologue::{
        ClientError, ErrorStatus, Header, Informational, Method, Protocol, Redirection,
        ServerError, Status, Successful, client, contains_header, err_stt, header_value, server,
        status,
    };
}

pub mod uri {
    pub use pheasant_uri::{Origin, Query, Resource, Url};
}

pub mod services {
    pub use pheasant_services::{
        Cors, Forward, MessageBodyInfo, Ranges, ReadCookies, Resource, Server, Service,
        WriteCookies, client_socket, cors, date, http_error, parse, print, server_socket,
        support_ranges,
    };
}

pub mod socket {
    pub use pheasant_socket::{
        AddressFamily, Error, ProtocolNumber, SocketLevel, SocketType,
        address::{InAddr, SockAddrIn, SockAddrUn},
        socket::{
            GetSockOpts, RecvFlags, SendFlags, SetSockOpts, SockAddrCasting, Socket, SocketOption,
            TrueSockAddr, VoidCasting, linger,
        },
    };
}
