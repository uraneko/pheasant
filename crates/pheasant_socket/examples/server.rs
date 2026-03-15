use pheasant_socket::{
    AddressFamily, Error, ProtocolNumber, SocketType,
    address::inet::SockAddrIn,
    socket::{SetSockOpts, Socket},
};

fn main() -> Result<(), Error> {
    let socket = Socket::new(
        AddressFamily::Inet,
        SocketType::SockStream,
        ProtocolNumber::Default,
    )?;

    println!("{:?}", socket);
    SetSockOpts::new(socket.fd()).reuse_address(true)?;

    // let addr = SockAddrIn::new([127, 0, 10, 1], 9988);
    let Ok(addr) = "127.0.10.1:9988".parse() else {
        panic!("bad address value or parser")
    };
    println!("{:?}", addr);
    let mut socket = socket.init::<SockAddrIn>(addr);
    // let sa_in = SockAddrIn::new(AddressFamily::Inet, c"127.0.10.1", u16::from_be(9988));
    socket.bind()?;
    socket.listen(4096)?;
    println!("\x1b[1;34mhttp://127.0.10.1:9988\x1b[0m");
    let client = socket.accept()?;

    let mut req = [0u8; 4096];
    socket.recv(client.fd(), &mut req, 0)?;
    println!("{}", unsafe { str::from_utf8_unchecked(&req) });

    let resp = b"HTTP/1.1 200 OK\ncontent-length: 38\ncontent-type: text/plain\n\ni now write back to the client socket";
    socket.send(client.fd(), resp, 0)?;

    Ok(())
}
