#![no_std]

pub mod http {
    pub use pheasant_http::{
        ClientError, ErrorStatus, Header, Informational, Method, Protocol, Redirection, Request,
        Respond, ServerError, Status, Successful, contains_header, err_stt, header_value, status,
    };
}

pub mod uri {
    pub use pheasant_uri::{Origin, Query, Resource, Url};
}

pub mod services {
    pub use pheasant_services::{
        Blacklist, Content, Cors, Forward, GateWay, Ranges, ReadCookies, Resource, Server, Service,
        Whitelist, WriteCookies, cors, date, http_error, print, request, respond, socket,
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
