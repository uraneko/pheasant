use pheasant::http::{ErrorStatus, Method, Protocol, Respond, err_stt, status};
use pheasant::services::{
    Server, Socket, bad_request, parse, read_stream, req_buf, resp_write_stream,
};
use std::io::BufReader;

mod services;
use services::*;

#[tokio::main]
pub async fn main() -> Result<(), ErrorStatus> {
    let Ok(mut socket) = Socket::builder([127, 10, 10, 1], 80)
        .buf_size(4096)
        .database("data/ciphr.db3")
        .build()
    else {
        return err_stt!(?500);
    };

    socket.init_message();
    socket
        .event_loop(async |this: &mut Socket| {
            {
                let mut resp = Respond::new(Protocol::Http11, status!(200));
                while let Ok((mut stream, _)) = read_stream(&this.socket) {
                    resp.clear();
                    let mut reader = BufReader::new(&mut stream);
                    let Ok(req_buf) = req_buf(&mut reader) else {
                        bad_request(&mut resp);
                        resp_write_stream(&resp, &mut stream, Method::Get)?;
                        continue;
                    };
                    let req = parse(req_buf);
                    let Ok(req) = req else {
                        bad_request(&mut resp);
                        resp_write_stream(&resp, &mut stream, Method::Get)?;
                        continue;
                    };
                    let method = req.method();

                    // lookup should fetch whole service chains
                    let service = match lookup(&req, &mut resp) {
                        Ok(s) => s,
                        Err(_err) => {
                            bad_request(&mut resp);
                            resp_write_stream(&resp, &mut stream, req.method())?;
                            continue;
                        }
                    };
                    this.service(req, &mut resp, service).await?;
                    resp_write_stream(&resp, &mut stream, method)?;
                }

                Ok(())
            }
        })
        .await?;

    Ok(())
}

// fn load_services() {
//     let funs = std::collections::HashMap::from([
//         ("favicon.ico", Services::Favicon),
//         ("GET /", Services::Index),
//     ]);
// }
