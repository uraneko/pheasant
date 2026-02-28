use pheasant_iptcp::*;

fn main() {
    let sockfd =
        AcquireSockFd::new(Domain::AfInet, SocketType::SockStream, Default::default()).acquire();

    println!("socketfd -> {}", sockfd);
}
