// values gotten from /usr/include/bits/socket.h
// under /* Protocol families.  */ definitions
// AF_* definitions are just aliases for the PF_* definitions there
// DOCS using Af* syntax instead of Pf since 'https://linux.die.net/man/2/socket' claims:
// However, already the BSD man page promises:
// "The protocol family generally is the same as the address family",
// and subsequent standards use AF_* everywhere.
#[derive(Debug, Clone, Copy)]
pub enum AddressFamily {
    // for disconnecting, at least on linux for afinet
    Unspec,
    // for local communications
    Unix, // aka AF_LOCAL
    // for ipv4
    Inet,
    // for ipv6
    Inet6,
    Ipx,
    NetLink,
    X25,
    Ax25,
    Atmpvc,
    Appletalk,
    Packet,
}

impl From<AddressFamily> for i32 {
    fn from(pf: AddressFamily) -> i32 {
        // WARN dont do this anymore:
        // ```
        // use Enum::*;
        // match self {
        //     Variant1 => 1,
        //     Variant2 => ....
        // }
        // ```
        // any wrongly typed variant name would end the match as the compiler mistakes
        // the mistyped name for an arm that catches all possible match values
        match pf {
            AddressFamily::Unspec => 0,
            AddressFamily::Unix => 1,
            AddressFamily::Inet => 2,
            AddressFamily::Inet6 => 10,
            AddressFamily::Ipx => 4,
            AddressFamily::NetLink => 16,
            AddressFamily::X25 => 9,
            AddressFamily::Ax25 => 3,
            AddressFamily::Atmpvc => 8,
            AddressFamily::Appletalk => 5,
            AddressFamily::Packet => 17,
        }
    }
}

impl From<AddressFamily> for u16 {
    fn from(pf: AddressFamily) -> u16 {
        // WARN dont do this anymore:
        // ```
        // use Enum::*;
        // match self {
        //     Variant1 => 1,
        //     Variant2 => ....
        // }
        // ```
        // any wrongly typed variant name would end the match as the compiler mistakes
        // the mistyped name for a variable that catches the match value
        match pf {
            AddressFamily::Unspec => 0,
            AddressFamily::Unix => 1,
            AddressFamily::Inet => 2,
            AddressFamily::Inet6 => 10,
            AddressFamily::Ipx => 4,
            AddressFamily::NetLink => 16,
            AddressFamily::X25 => 9,
            AddressFamily::Ax25 => 3,
            AddressFamily::Atmpvc => 8,
            AddressFamily::Appletalk => 5,
            AddressFamily::Packet => 17,
        }
    }
}

// values retrieved from bits/socket_type.h
// under section /* Types of sockets.  */
#[derive(Debug, Clone, Copy)]
pub enum SocketType {
    // use this to open a tcp socket
    Stream,
    // use this to open a udp socket
    Dgram,
    SeqPacket,
    // use this for direct access to the underlying ip protocol
    Raw,
    Rdm,
    // Deprecated
    // use (afpacket, sock*, 0)
    // SockPacket
}
// sock type conv fail
#[derive(Debug)]
pub enum ConversionError {
    BadInt(i32),
}

impl TryFrom<i32> for SocketType {
    type Error = ConversionError;
    fn try_from(int: i32) -> Result<Self, Self::Error> {
        Ok(match int {
            1 => Self::Stream,
            2 => Self::Dgram,
            3 => Self::Raw,
            4 => Self::Rdm,
            5 => Self::SeqPacket,
            int => return Err(ConversionError::BadInt(int)),
        })
    }
}

impl From<SocketType> for i32 {
    fn from(st: SocketType) -> i32 {
        match st {
            SocketType::Stream => 1,
            SocketType::Dgram => 2,
            SocketType::Raw => 3,
            SocketType::Rdm => 4,
            SocketType::SeqPacket => 5,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ProtocolNumber {
    #[default]
    Default,
    Icmp,
    Ipv4,
    Ipv6,
    Tcp,
    Udp,
    Else(i32),
}

impl From<ProtocolNumber> for i32 {
    fn from(pn: ProtocolNumber) -> i32 {
        match pn {
            ProtocolNumber::Default => 0,
            ProtocolNumber::Ipv4 => 4,
            ProtocolNumber::Ipv6 => 41,
            ProtocolNumber::Tcp => 6,
            ProtocolNumber::Udp => 17,
            ProtocolNumber::Icmp => 1,
            ProtocolNumber::Else(num) => num,
        }
    }
}

impl TryFrom<i32> for ProtocolNumber {
    type Error = ConversionError;
    fn try_from(int: i32) -> Result<Self, Self::Error> {
        Ok(match int {
            0 => Self::Default,
            4 => Self::Ipv4,
            41 => Self::Ipv6,
            6 => Self::Tcp,
            17 => Self::Udp,
            1 => Self::Icmp,
            int => return Err(ConversionError::BadInt(int)),
        })
    }
}

#[repr(C)]
pub enum SocketLevel {
    // this sets/gets the option for the socket level itself not a deeper protocol
    // whatever that means
    Socket = 1,
    Ip = 0,
    Ipv6 = 41,
    Icmpv6 = 58,
    Raw = 255,
}

impl From<SocketLevel> for i32 {
    fn from(sl: SocketLevel) -> i32 {
        match sl {
            SocketLevel::Ip => 0,
            SocketLevel::Socket => 1,
            SocketLevel::Ipv6 => 41,
            SocketLevel::Icmpv6 => 58,
            SocketLevel::Raw => 255,
        }
    }
}

pub const INADDR_ANY: u32 = 0;
