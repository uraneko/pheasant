use core::ffi::c_void;
use pheasant_sys::*;

fn main() {
    let sockfd = AcquireSockFd::new(AddressFamily::AfInet, SocketType::SockStream, 0).acquire();
    println!("socketfd -> {}", sockfd);
    println!("{}", std::io::Error::last_os_error());

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
                0,
                SocketOption::SndBuf.into_int(),
                &mut val as *mut i32 as *mut c_void,
                &mut size as *mut usize as *mut u32
            ),
            std::io::Error::last_os_error()
        );
    }
    println!("buf size is {}", val);

    unsafe {
        println!(
            "err {} {{ {} }}",
            setsockopt(
                sockfd,
                1,
                SocketOption::SndBuf.into_int(),
                ptr,
                core::mem::size_of_val(&ptr) as u32
            ),
            std::io::Error::last_os_error()
        );

        println!(
            "err {} {{ {} }}",
            setsockopt(
                sockfd,
                1,
                SocketOption::ReuseAddr.into_int(),
                &1 as *const i32 as *const c_void,
                core::mem::size_of_val(&buf_size) as u32
            ),
            std::io::Error::last_os_error()
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
                SocketOption::SndBuf.into_int(),
                &mut val as *mut i32 as *mut c_void,
                &mut size as *mut usize as *mut u32
            ),
            std::io::Error::last_os_error()
        );
    }
    println!("buf size is {}", val);

    let sa_in = SockAddrIn::new(AddressFamily::AfInet, c"127.0.10.1", u16::from_be(9988));
    println!(
        "err {} {{ {} }}",
        BindSocketToAddr::new(sockfd, sa_in).bind(),
        std::io::Error::last_os_error()
    );
    let mut listening = 0;
    unsafe {
        println!(
            "err {} {{ {} }} \\ are we listening -> {}",
            getsockopt(
                sockfd,
                1,
                SocketOption::AcceptConn.into_int(),
                &mut listening as *mut i32 as *mut c_void,
                &mut size as *mut usize as *mut u32
            ),
            std::io::Error::last_os_error(),
            if listening == 0 { false } else { true }
        );
    }

    println!(
        ">>>> err {} {{ {} }}",
        ListenOnSocket::new(sockfd, 5).listen(),
        std::io::Error::last_os_error()
    );
    let mut listening = 0;
    unsafe {
        println!(
            "err {} {{ {} }} \\ are we listening -> {}",
            getsockopt(
                sockfd,
                1,
                SocketOption::AcceptConn.into_int(),
                &mut listening as *mut i32 as *mut c_void,
                &mut size as *mut usize as *mut u32
            ),
            std::io::Error::last_os_error(),
            if listening == 0 { false } else { true }
        );
    }

    let mut sa_in = SockAddrIn::new(AddressFamily::AfInet, c"0.0.0.0", u16::from_be(0));
    let mut len = SockAddrIn::SIZE;
    println!("\x1b[1;34mhttp://127.0.10.1:9988\x1b[0m");

    // println!(
    //     "err {} {{ {} }}",
    //     unsafe { close(sockfd) },
    //     std::io::Error::last_os_error()
    // );

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
    println!("addr: {:?}\nlen: {}", sa_in, len);

    let mut req = [0u8; 4096];
    unsafe {
        // use RecvFlag::*;

        // println!(
        //     "read up to {} bytes",
        //     recv(
        //         clisockfd,
        //         &mut req as *mut [u8] as *mut c_void,
        //         10,
        //         RecvFlag::union(&[MsgErrqueue, MsgDontwait])
        //     )
        // );
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
    println!("{:?}", &req[..1866]);
    println!(
        "err {} {{ {} }}",
        unsafe { shutdown(sockfd, Shutdown::Write.into()) },
        std::io::Error::last_os_error()
    );

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
