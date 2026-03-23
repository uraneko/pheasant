use pheasant_socket::{
    AddressFamily, Error, ProtocolNumber, SocketType, address::unix::SockAddrUn, socket::Socket,
};

fn main() -> Result<(), Error> {
    let socket = Socket::new(
        AddressFamily::Unix,
        SocketType::Stream,
        ProtocolNumber::Default,
    )?;

    // SetSockOpts::new(socket.fd()).reuse_address(true)?;

    let Ok(addr) = "socket13".parse::<SockAddrUn>() else {
        panic!("bad address value or parser");
    };
    let mut socket = socket.init(addr);
    println!("{:?}", socket);
    socket.bind()?;
    std::thread::sleep(std::time::Duration::from_secs(7));
    socket.unlink()?;
    // socket.listen(4096)?;
    // println!("\x1b[1;34mhttp://127.0.10.1:9988\x1b[0m");
    // let client = socket.accept()?;
    //
    // let mut req = [0u8; 4096];
    // socket.recv(client.fd(), &mut req, 0)?;
    // println!("{}", unsafe { str::from_utf8_unchecked(&req) });
    //
    // let resp = b"HTTP/1.1 200 OK\ncontent-length: 38\ncontent-type: text/plain\n\ni now write back to the client socket";
    // socket.send(client.fd(), resp, 0)?;

    Ok(())
}
