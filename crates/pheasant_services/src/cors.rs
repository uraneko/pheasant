pub fn cors(resp: &mut String, status: &str) {
    let headers = "access-control-allow-headers: *\n";
    let origin = "access-control-allow-origin: 127.10.10.1:1024\n";
    let methods = "access-control-allow-methods: HEAD, GET, OPTIONS\n";
    *resp = format!("{}{}{}{}", status, headers, origin, methods);
}
