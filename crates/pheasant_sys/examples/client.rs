use core::ffi::c_void;
use pheasant_sys::*;

fn main() {
    let sockfd = AcquireSockFd::new(AddressFamily::Inet, SocketType::SockStream, 0).acquire();
    println!("socketfd -> {}", sockfd);
    println!("{}", std::io::Error::last_os_error());

    // NOTE this number would be doubled on success
    // i.e., your actual buf size would be buf_size * 2
    // WARN if buf_size < 2304 setsockopt would force buf_size = 2304;
    // -> which would make you actual buf size 4608 bytes

    unsafe {
        println!(
            "err {} {{ {} }}",
            setsockopt(
                sockfd,
                1,
                SocketOption::ReuseAddr.into_int(),
                &1 as *const i32 as *const c_void,
                4
            ),
            std::io::Error::last_os_error()
        );
    }

    let sa_in = SockAddrIn::new(AddressFamily::Inet, c"127.0.10.1", u16::from_be(9988));
    let len = SockAddrIn::SIZE;

    unsafe {
        println!(
            "err {} {{ {} }}",
            connect(sockfd, &sa_in as *const SockAddrIn as *const SockAddr, len,),
            std::io::Error::last_os_error()
        );
    }

    let resp = b"HTTP/1.1 200 OK\ncontent-length: 26\ncontent-type: text/plain\n\nit is i, the client socket";
    unsafe {
        println!(
            "wrote up to {} bytes",
            send(
                sockfd,
                resp as *const [u8] as *const c_void,
                resp.len() as u64,
                0
            )
        );
    }

    let mut req = [0u8; 4096];
    unsafe {
        println!(
            "read up to {} bytes",
            recv(
                sockfd,
                &mut req as *mut [u8] as *mut c_void,
                req.len() as u64,
                0
            )
        );
    }
    println!("{}", unsafe { str::from_utf8_unchecked(&req) });
}
