use super::*;

// fn error(err: i32) -> Error {
// if [1, 4, 11, 14, 22, 23, 24, 71, 77, 88, 95, 103, 105].contains(&err) {
// return err.into();
// }

pub struct Accept<'a, A: SockAddrCasting> {
    // server's socket fd
    server: &'a Socket<A>,
}

impl<'a, A: SockAddrCasting + Clone> Accept<'a, A> {
    pub fn accept(&self) -> Result<Socket<A>, Error> {
        let mut peer_addr = self.server.address.clone();
        let mut size = A::SIZE;
        match unsafe {
            accept(
                self.server.fd() as i32,
                peer_addr.cast_mut(),
                &mut size as *mut u32,
            )
        } {
            -1 => {
                extern crate std;

                Err(std::io::Error::last_os_error().raw_os_error().into())
            }
            fd if fd > 0 => Ok(Socket::new(fd as u32, peer_addr, self.server.params)),
            _ => unreachable!("man page says result should be -1 or > 0"),
        }
    }
}
