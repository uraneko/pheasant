use core::ffi::c_uchar;

pub mod inet;
pub use inet::{InAddr, SockAddrIn};

type c_uint = u16;

// c_uint repr of an address family
// WARN the address_family int must be 16 bits sized
// or else the functions that take a sockaddr ptr would misunderstand
// the passed address boundaries of the sockaddr_* struct
// WARN (move this somewhere else) the address port mst be passed in in big endianne notation
// else the socket would bind to the be repr of your le number
#[allow(non_camel_case_types)]
type sa_family_t = c_uint;

// WARN assuming i got the documentation write
// this type should be a trait implemented by
// the various socket address families structs
#[repr(C)]
#[derive(Debug)]
pub struct SockAddr {
    // use AddressFamily::ChoiceFamily.as_int() to populate this field
    // as it refers to a c_uint repr of the address family value
    sa_family: sa_family_t,
    sa_data: [c_uchar; 14],
}

#[allow(non_camel_case_types)]
pub type uint32_t = u32;
// alias of uint32_t
#[allow(non_camel_case_types)]
pub type in_addr_t = uint32_t;

impl From<SockAddrIn> for SockAddr {
    fn from(sa_in: SockAddrIn) -> SockAddr {
        let [o0, o1, o2, o3] = sa_in.sin_addr.s_addr.to_be_bytes();
        let data = format!("{}.{}.{}.{}:{}", o0, o1, o2, o3, sa_in.sin_port)
            .into_bytes()
            .try_into()
            .unwrap();

        Self {
            sa_family: sa_in.sin_family,
            sa_data: data,
        }
    }
}

impl SockAddr {
    pub const SIZE: u32 = core::mem::size_of::<Self>() as u32;
    pub fn new(sa_family: c_uint, data: &str) -> Self {
        Self {
            sa_family,
            sa_data: data.as_bytes().try_into().unwrap(),
        }
    }
}

#[repr(C)]
pub struct SockAddrStorage {
    // address family int repr
    ss_family: sa_family_t,
}

#[cfg(test)]
mod sizes {
    use super::{SockAddr, SockAddrIn};

    #[test]
    fn sockaddr_in() {
        assert_eq!(SockAddr::SIZE, SockAddrIn::SIZE);
    }
}
