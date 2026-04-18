pub mod server {
    use crate::{Request, Respond};
    use embedded_io::{Read, Write};

    pub fn print_resp(resp: &Respond) {
        let resp = resp.server_ref();
        println!(
            "{} {} {}",
            resp.proto().as_str(),
            resp.status().code(),
            resp.status().text()
        );
        let headers: pheasant_http::headers::HeadersRef = resp.headers().into();
        let headers = headers.stream_bytes();
        println!(
            "{}",
            Box::<str>::from_iter(headers.into_iter().map(|b| b as char))
        );
        // let n = resp.read_body(buf).unwrap();
        let data = str::from_utf8(resp.body()).unwrap_or_else(|_| "body err".into());
        if data.len() > 128 {
            println!("{}...", &data[..128]);
        } else {
            println!("{}", data);
        }
        println!("***");
    }

    pub fn print_req(req: &Request) {
        let req = req.server_ref();
        println!(
            "{} - {} - {:?} - {}",
            req.method(),
            req.path_str(),
            req.query(),
            req.proto(),
        );
        req.headers()
            .iter()
            .inspect(|h| {
                println!(
                    "{} -> {}",
                    str::from_utf8(h.field_ref()).unwrap_or_else(|_| "field err".into()),
                    str::from_utf8(h.value_ref()).unwrap_or_else(|_| "value err".into())
                )
            })
            .count();
        if let Some(body) = req.body() {
            println!(
                "\n{}",
                str::from_utf8(body).unwrap_or_else(|_| "body err".into())
            );
        }
        println!("---");
    }
}
