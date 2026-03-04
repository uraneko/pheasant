use core::ffi::c_void;
use pheasant_iptcp::*;

fn main() {
    let sockfd = AcquireSockFd::new(AddressFamily::PfInet, SocketType::SockStream, 0).acquire();
    println!("socketfd -> {}", sockfd);

    let buf_size: i32 = 2048;
    let ptr = &buf_size as *const i32 as *const c_void;
    unsafe {
        println!("{:?} -> {}", ptr, *(ptr as *const i32));
    }
    unsafe {
        println!(
            "err {} {{ {} }}",
            setsockopt(
                sockfd,
                1,
                SocketOption::SO_SNDBUF.as_int(),
                ptr,
                core::mem::size_of_val(&buf_size) as u32
            ),
            errno::errno()
        );

        println!(
            "err {} {{ {} }}",
            setsockopt(
                sockfd,
                1,
                SocketOption::SO_REUSEADDR.as_int(),
                &1 as *const i32 as *const c_void,
                core::mem::size_of_val(&buf_size) as u32
            ),
            errno::errno()
        );
    }

    let sa_in = SockAddrIn::new(AddressFamily::PfInet, c"127.0.10.1", u16::from_be(9988));
    println!(
        "err {} {{ {} }}",
        BindSocketToAddr::new(sockfd, sa_in).bind(),
        errno::errno()
    );
    println!(
        "err {} {{ {} }}",
        ListenOnSocket::new(sockfd, 5).listen(),
        errno::errno()
    );
    let mut sa_in = SockAddrIn::new(AddressFamily::PfInet, c"0.0.0.0", u16::from_be(0));
    let mut len = SockAddrIn::SIZE;
    unsafe {
        println!(
            "err {} {{ {} }}",
            accept(
                sockfd,
                &mut sa_in as *mut SockAddrIn as *mut SockAddr,
                &mut len as *mut u32,
            ),
            errno::errno()
        );
    }
    println!("addr: {:?}\nlen: {}", sa_in, len);
    std::thread::sleep(std::time::Duration::from_secs(5));
}
