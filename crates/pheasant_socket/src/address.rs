pub mod inet;
// pub mod inet6;
// pub mod unix;

pub use inet::{InAddr, SockAddrIn};
/// c_uint repr of an address family
/// WARN the address_family int must be 16 bits sized
/// or else the functions that take a sockaddr ptr would misunderstand
/// the passed address boundaries of the sockaddr_* struct
/// WARN (move this somewhere else) the address port mst be passed in in big endianne notation
/// else the socket would bind to the be repr of your le number

#[repr(C)]
#[derive(Debug)]
pub struct SockAddr {
    // use AddressFamily::ChoiceFamily.as_int() to populate this field
    // as it refers to a c_uint repr of the address family value
    sa_family: u16,
    sa_data: [u8; 14],
}

impl SockAddr {
    pub const SIZE: u32 = core::mem::size_of::<Self>() as u32;
    pub fn new(sa_family: u16, data: &str) -> Self {
        Self {
            sa_family,
            sa_data: data.as_bytes().try_into().unwrap(),
        }
    }
}

// handles both ipv4 and 6
// #[repr(C)]
// pub struct SockAddrStorage {
//     // address family int repr
//     ss_family: u16,
// }

#[cfg(test)]
mod sizes {
    use super::{SockAddr, SockAddrIn};

    #[test]
    fn sockaddr_in() {
        assert_eq!(SockAddr::SIZE, SockAddrIn::SIZE);
    }
}
