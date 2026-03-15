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

    let Ok(srvr_addr) = "127.0.10.1:9988".parse::<SockAddrIn>() else {
        panic!("bad address value or parser")
    };
    socket.connect(&srvr_addr)?;
    // let sa_in = SockAddrIn::new(AddressFamily::Inet, c"127.0.10.1", u16::from_be(9988));

    let msg = b"HTTP/1.1 200 OK\ncontent-length: 26\ncontent-type: text/plain\n\nit is i, the client socket";
    let _n = socket.send(msg, 0)?;
    // println!("sent {} octets", n);

    let mut resp = [0u8; 4096];
    let _ = socket.recv(&mut resp, 0)?;
    // println!("received {} octets", n);
    println!("{}", unsafe { str::from_utf8_unchecked(&resp) });

    Ok(())
}
