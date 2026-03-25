pub mod server {
    use embedded_io::{Read, Write};
    use pheasant_prologue::server::{Request, Respond};

    pub fn print_resp<H: Read + Write, B: Read + Write>(resp: &mut Respond<H, B>, buf: &mut [u8]) {
        println!(
            "{} {} {}",
            resp.proto_cpy().as_str(),
            resp.status_cpy().code(),
            resp.status_cpy().text()
        );
        let n = resp.read_headers(buf).unwrap();
        println!(
            "{}",
            str::from_utf8(&buf[..n]).unwrap_or_else(|_| "headers err".into())
        );
        let n = resp.read_body(buf).unwrap();
        println!(
            "{}",
            str::from_utf8(&buf[..n]).unwrap_or_else(|_| "body err".into())
        );
        println!("***");
    }

    pub fn print_req(req: &Request) {
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
