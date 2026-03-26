#![no_std]

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
