use pheasant_core::{Protocol, Status, status};
use pheasant_headers::{CorsConfigs, Encoding};
use pheasant_server::{
    request::Request, resource::Resource, respond::Respond, servlet::Servlet, socket::HttpSocket,
};

#[tokio::main]
async fn main() {
    let servlet = Servlet::builder(index).query(true).build();
    let resource = Resource::builder("/index.html")
        .forward("/")
        .get(servlet)
        .head(true)
        .build();
    let mut socket: HttpSocket = HttpSocket::builder([127, 0, 0, 1], 7070)
        .unwrap()
        .resource(resource)
        .buf_size1(4096)
        .build()
        .unwrap();
    socket.init_message();
    socket.fireup().await.unwrap();
}

async fn index(req: Request) -> Respond {
    let body = format!("<h1>Hello {}</h1>", req.param("who").unwrap_or("wakanda"));
    let resp = Respond::builder(status!(200), Protocol::Http11, false)
        .body(body)
        .date()
        .server("pheasant0.1/dev mode")
        .content_type("text/html")
        .content_length();
    // .content_encoding(Encoding::Deflate)
    // .encoding(Encoding::Gzip)
    // .encode();

    resp.build().unwrap()
}
