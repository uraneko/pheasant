use core::ffi::c_void;
use pheasant_sys::*;

fn main() {
    let sockfd = AcquireSockFd::new(
        AddressFamily::PfInet,
        SocketType::SockStream,
        ProtocolNumber::Tcp,
    )
    .acquire();
    println!("socketfd -> {}", sockfd);

    // NOTE this number would be doubled on success
    // i.e., your actual buf size would be buf_size * 2
    // WARN if buf_size < 2304 setsockopt would force buf_size = 2304;
    // -> which would make you actual buf size 4608 bytes
    let buf_size: i32 = 2306;
    let ptr = &buf_size as *const i32 as *const c_void;
    unsafe {
        println!("{:?} -> {}", ptr, *(ptr as *const i32));
    }

    let mut val = 0i32;
    let mut size = core::mem::size_of_val(&buf_size);
    unsafe {
        println!(
            "err {} {{ {} }}",
            getsockopt(
                sockfd,
                1,
                SocketOption::SO_SNDBUF.as_int(),
                &mut val as *mut i32 as *mut c_void,
                &mut size as *mut usize as *mut u32
            ),
            errno::errno()
        );
    }
    println!("buf size is {}", val);

    unsafe {
        println!(
            "err {} {{ {} }}",
            setsockopt(
                sockfd,
                1,
                SocketOption::SO_SNDBUF.as_int(),
                ptr,
                core::mem::size_of_val(&ptr) as u32
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

    let mut val = 0i32;
    let mut size = core::mem::size_of_val(&buf_size);
    unsafe {
        println!(
            "err {} {{ {} }}",
            getsockopt(
                sockfd,
                1,
                SocketOption::SO_SNDBUF.as_int(),
                &mut val as *mut i32 as *mut c_void,
                &mut size as *mut usize as *mut u32
            ),
            errno::errno()
        );
    }
    println!("buf size is {}", val);

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
    println!("\x1b[1;34mhttp://127.0.10.1:9988\x1b[0m");
    let clisockfd = unsafe {
        accept(
            sockfd,
            &mut sa_in as *mut SockAddrIn as *mut SockAddr,
            &mut len as *mut u32,
        )
    };
    println!("err {} {{ {} }}", clisockfd, errno::errno());
    println!("addr: {:?}\nlen: {}", sa_in, len);

    let mut req = [0u8; 512];
    unsafe {
        use RecvFlag::*;

        println!(
            "read up to {} bytes",
            recv(
                clisockfd,
                &mut req as *mut [u8] as *mut c_void,
                10,
                RecvFlag::union(&[MsgErrqueue, MsgDontwait])
            )
        );
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
    println!("{:?}", str::from_utf8(&req));

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

    // std::thread::sleep(std::time::Duration::from_secs(5));
}
