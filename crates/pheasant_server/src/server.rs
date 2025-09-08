use hashbrown::HashSet;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};

use super::{
    ClientError, Failure, HttpSocket, Method, PheasantError, PheasantResult, Protocol, Redirection,
    Request, Response, ResponseStatus, Route, ServerError, Service, ServiceBundle, Status,
    Successful,
};

pub struct Server {
    sockets: HashSet<HttpSocket>,
}

// WARN when responding to a credentialed request, the CORS glob/* header value is not allowed for the following headers
// Access-Control-Allow-Origin, Access-Control-Allow-Headers, Access-Control-Allow-Methods and Access-Control-Expose-Headers
// TODO Server.origins { whitelist, blacklist }

// #[deprecated(note = "replaced by Request::from_stream")]
// async fn read_stream(s: &mut TcpStream) -> PheasantResult<String> {
//     let mut data = Vec::new();
//     let mut reader = BufReader::new(s);
//     let mut buf = [0; 1024];
//     loop {
//         let Ok(n) = reader.read(&mut buf) else {
//             return Err(PheasantError::StreamReadCrached);
//         };
//         if n < 1024 {
//             break data.extend(&buf[..n]);
//         } else if n > 1024 {
//             return Err(PheasantError::StreamReadWithExcess);
//         }
//         data.extend(buf);
//     }
//
//     String::from_utf8(data).map_err(|e| e.into())
// }

// #[deprecated(note = "replaced by Response::respond")]
// fn format_response(payload: Vec<u8>, ct: &Mime) -> Vec<u8> {
//     let cl = payload.len();
//     let mut res: Vec<u8> = format!(
//         "HTTP/1.1 200 OK\r\nAccept-Range: bytes\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
//         ct, cl
//     )
//     .into_bytes();
//     res.extend(payload);
//     res.extend([13, 10]);
//
//     res
// }

impl From<&Request> for () {
    fn from(_: &Request) -> () {
        ()
    }
}
