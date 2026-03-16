use pheasant::prologue::{ErrorStatus, Method, Protocol, err_stt, server::Respond, status};
use pheasant::services::{Server, http_error, parse, server_socket::Socket};

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
        resp.clear();
        server.read(client.fd()).map_err(|_| err_stt!(500))?;
        let req_buf = &server.buf_ref();
        let Ok(req) = parse(req_buf) else {
            http_error(err_stt!(400), &mut resp);
            server.dump(&resp, Method::Get);
            server.write(client.fd()).map_err(|_| err_stt!(500))?;

            continue;
        };
        server.clr_buf();
        let method = req.method();
        let service = match lookup(&req.path_str()) {
            Err(err) => {
                http_error(err, &mut resp);
                server.dump(&resp, method);
                server.write(client.fd()).map_err(|_| err_stt!(500))?;

                continue;
            }
            Ok(service) => service,
        };
        server.service(req, &mut resp, service).await?;
        server.dump(&resp, method);
        server.write(client.fd()).map_err(|_| err_stt!(500))?;

        // this shuts down the while loop
        // sock.shutdown_read().map_err(|_| err_stt!(500))?;
    }

    Ok(())
}
