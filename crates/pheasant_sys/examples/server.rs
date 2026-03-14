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
    println!(
        "err {} {{ {} }}",
        BindSocketToAddr::new(sockfd, sa_in).bind(),
        std::io::Error::last_os_error()
    );

    println!(
        ">>>> err {} {{ {} }}",
        ListenOnSocket::new(sockfd, 5).listen(),
        std::io::Error::last_os_error()
    );

    let mut sa_in = SockAddrIn::new(AddressFamily::Inet, c"0.0.0.0", u16::from_be(0));
    let mut len = SockAddrIn::SIZE;
    println!("\x1b[1;34mhttp://127.0.10.1:9988\x1b[0m");

    let clisockfd = unsafe {
        accept(
            sockfd,
            &mut sa_in as *mut SockAddrIn as *mut SockAddr,
            &mut len as *mut u32,
        )
    };
    println!("peer-addr: {:?}", sa_in);
    println!(
        "err {} {{ {} }}",
        clisockfd,
        std::io::Error::last_os_error()
    );

    let mut req = [0u8; 4096];
    unsafe {
        println!(
            "read up to {} bytes",
            recv(
                clisockfd,
                &mut req as *mut [u8] as *mut c_void,
                req.len() as u64,
                0
            )
        );
    }
    println!("{}", unsafe { str::from_utf8_unchecked(&req) });
    let resp = b"HTTP/1.1 200 OK\ncontent-length: 32\ncontent-type: text/plain\n\ni now write to the client socket";
    unsafe {
        println!(
            "wrote up to {} bytes",
            send(
                clisockfd,
                resp as *const [u8] as *const c_void,
                resp.len() as u64,
                0
            )
        );
    }
}
