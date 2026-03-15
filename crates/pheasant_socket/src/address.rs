pub use pheasant_sys::socket::SockAddr;

pub mod inet;
// pub mod inet6;
pub mod unix;

pub use inet::{InAddr, SockAddrIn};
pub use unix::SockAddrUn;
/// c_uint repr of an address family
/// WARN the address_family int must be 16 bits sized
/// or else the functions that take a sockaddr ptr would misunderstand
/// the passed address boundaries of the sockaddr_* struct
/// WARN (move this somewhere else) the address port mst be passed in in big endianne notation
/// else the socket would bind to the be repr of your le number

#[cfg(test)]
mod sizes {
    use super::{SockAddr, SockAddrIn, SockAddrUn};

    #[test]
    fn sockaddr_in() {
        assert_eq!(SockAddr::SIZE, SockAddrIn::SIZE);
    }

    #[test]
    fn sockaddr_un() {
        assert_eq!(SockAddr::SIZE, SockAddrUn::SIZE);
    }
}
