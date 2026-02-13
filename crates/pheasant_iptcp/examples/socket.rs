use pheasant_iptcp::*;

fn main() {
    unsafe {
        let sfd = socket(Domain::AfInet.into(), Type::SockStream.into(), 0);
        let mut addr = sockaddr_un {
            sun_family: AF_UNIX,
            sun_path: b"/home/brownbread/forge/.pool.txt".into(),
        };
        let res = bind(sfd, &mut addr as *mut sockaddr, sizeof::<sockaddr_un>());
    }
}

fn attempt1() {
    unsafe {
        let len = std::mem::size_of::<u32>() as u32;
        let sock_fd = socket(Domain::AfInet.into(), Type::SockStream.into(), 0);
        println!("sockfd: {}", sock_fd);

        // let mut addr = 0u32;
        // let addr_res = getsockopt(
        //     sock_fd,
        //     SOL_SOCKET,
        //     SO_REUSEADDR,
        //     &mut addr as *mut u32,
        //     len,
        // );
        // println!("get addrreuse {{ res: {}, val: {} }}", addr_res, addr);

        let optval = 1u32;
        let res = setsockopt(
            sock_fd,
            SOL_SOCKET,
            SO_REUSEADDR,
            // 1i16 as *const std::ffi::c_void
            &optval as *const u32,
            len,
        );
        println!("set addrreuse {{ res: {}, val: {} }}", res, optval);
        if res < 0 {
            perror(b"setsockopt failure:" as *const u8);
        }

        // let mut err = 0i16;
        // let err_res = getsockopt(sock_fd, SOL_SOCKET, SO_ERROR, &mut err as *mut i16, &len);
        // println!("get error {{ res: {}, val: {} }}", err_res, err);

        let mut addr = 0i32;
        let addr_res = getsockopt(
            sock_fd,
            SOL_SOCKET,
            SO_REUSEADDR,
            &mut addr as *mut i32,
            len,
        );
        // println!("get addrreuse {{ res: {}, val: {} }}", addr_res, addr);
        if addr_res < 0 {
            perror(b"getsockopt failure:" as *const u8);
        }
    }
}
