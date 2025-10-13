use pheasant_core::{Protocol, Status, status};
use pheasant_headers::{CorsConfigs, Encoding};
use pheasant_server::{Fallback, HttpSocket, Request, Resource, Respond, Servlet};

#[tokio::main]
async fn main() {
    let servlet = Servlet::builder(index).query(true).build();
    let fallback = Fallback::new(not_found, 404, None);
    let resource = Resource::builder("/index.html")
        .forward("/")
        .forward_status(301)
        .get(servlet)
        .head(true)
        .build();
    let mut socket: HttpSocket = HttpSocket::builder([127, 0, 0, 1], 7070)
        .unwrap()
        .resource(resource)
        .fallback(fallback)
        .buf_size(1024)
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
        .content_length()
        .content_encoding(Encoding::Deflate)
        .encode();

    resp.build().unwrap()
}

async fn not_found() -> Respond {
    Respond::builder(404.try_into().unwrap(), Protocol::Http11, false)
        .body("encountered http client error 404 - resource/method pair not found")
        .build()
        .unwrap()
}
