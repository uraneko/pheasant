use pheasant::prologue::{ErrorStatus, Method, Protocol, err_stt, server::Respond, status};
use pheasant::services::{Server, http_error, request, socket::server::Socket};

mod services;
use services::*;

#[tokio::main]
pub async fn main() -> Result<(), ErrorStatus> {
    let Ok(mut socket) = Socket::builder("127.0.10.1:7887")
        .map_err(|_| err_stt!(500))?
        .buf_size(4096)
        // .database("data/ciphr.db3")
        .build()
        .await
    else {
        return err_stt!(?500);
    };

    println!("{}", socket.init_message());
    socket.event_loop(hello_web).await?;

    Ok(())
}

async fn hello_web(server: &mut Socket) -> Result<(), ErrorStatus> {
    let mut resp = Respond::new(Protocol::Http11, status!(200));
    let sock = server.inner();
    while let Ok(client) = sock.accept() {
        println!("client = >{:?}<", client);
        let n = server.read(client.fd()).map_err(|_| err_stt!(500))?;
        let req_buf = &server.buf_ref()[..n];
        // println!("request = <{}>", unsafe {
        //     str::from_utf8_unchecked(req_buf)
        // });
        let Ok(req) = request(req_buf) else {
            http_error(err_stt!(400), &mut resp);
            server
                .write(client.fd(), &mut resp)
                .map_err(|_| err_stt!(500))?;

            continue;
        };
        let service = match lookup(&req.path_str()) {
            Err(err) => {
                http_error(err, &mut resp);
                server
                    .write(client.fd(), &mut resp)
                    .map_err(|_| err_stt!(500))?;

                continue;
            }
            Ok(service) => service,
        };
        server.service(req, &mut resp, service).await?;

        server
            .write(client.fd(), &mut resp)
            .map_err(|_| err_stt!(500))?;

        // shutdown the client so that it doesnt attempt to reuse the connection
        client.shutdown_readwrite().map_err(|_| err_stt!(500))?;
    }

    Ok(())
}
