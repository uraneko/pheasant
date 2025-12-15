use pheasant::http::{ErrorStatus, err_stt};
use pheasant::services::{Server, Socket, bad_request, parse, read_stream, req_buf, write_stream};
use std::io::BufReader;

mod services;
use services::*;

#[tokio::main]
pub async fn main() -> Result<(), ErrorStatus> {
    let Ok(mut socket) = Socket::builder([127, 10, 10, 1], 80)
        .buf_size(4096)
        .sqlite_path("data/ciphr.db3")
        .build()
    else {
        return err_stt!(?500);
    };

    socket.init_message();
    socket
        .event_loop(async |this: &mut Socket| {
            {
                let mut buf = Vec::new();
                while let Ok((mut stream, _)) = read_stream(&this.socket) {
                    buf.clear();
                    let mut reader = BufReader::new(&mut stream);
                    let Ok(req_buf) = req_buf(&mut reader) else {
                        bad_request(&mut buf);
                        write_stream(&buf, &mut stream);
                        continue;
                    };
                    let req = parse(req_buf);
                    let Ok(req) = req else {
                        bad_request(&mut buf);
                        write_stream(&buf, &mut stream);
                        continue;
                    };

                    // lookup should fetch whole service chains
                    let service = match lookup(&req, &mut buf) {
                        Ok(s) => s,
                        Err(_err) => {
                            bad_request(&mut buf);
                            write_stream(&buf, &mut stream);
                            continue;
                        }
                    };
                    this.service(req, &mut buf, service).await?;
                    write_stream(&buf, &mut stream);
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
