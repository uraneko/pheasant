use pheasant::http::{ErrorStatus, err_stt, request::Request};
use pheasant::services::{Cors, Resource, Service, Socket, not_found};

pub enum Services {
    Index(Index),
    Icon(Icon),
    CreateTable,
    Error,
}

impl Service<Socket> for Services {
    async fn run(
        &self,
        socket: &mut Socket,
        req: Request,
        buf: &mut Vec<u8>,
    ) -> Result<(), ErrorStatus> {
        match self {
            Self::Index(index) => index.run(socket, req, buf).await,
            Self::Icon(icon) => icon.run(socket, req, buf).await,
            Self::CreateTable => CreateTable.run(socket, req, buf).await,
            Self::Error => Ok(()),
        }
    }
}

pub struct Index {
    background: String,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            background: "#18191B".into(),
        }
    }
}

// nice crimsonish color #ab1243

impl Index {
    fn new(req: &Request) -> Self {
        if let Some(query) = req.query()
            && let Some(background) = query.param("background")
        {
            return Self {
                background: background.into(),
            };
        }

        Self::default()
    }
}

impl Resource<Socket> for Index {
    async fn get(
        &self,
        _socket: &mut Socket,
        _req: Request,
        buf: &mut Vec<u8>,
    ) -> Result<(), ErrorStatus> {
        let html0 = "<!DOCTYPE html><html><body style='background:";
        let html1 = ";color:beige'><h1>hello web</h1></body></html>";
        let len = html0.len() + html1.len() + self.background.len();
        buf.extend(
            format!(
                "HTTP/1.1 200 Ok\nContent-Type:text/html\nContent-Length:{}\n\n{}{}{}",
                len, html0, self.background, html1
            )
            .as_bytes(),
        );

        Ok(())
    }
}

// slice!("HTTP/1.1 200 OK\ncontent-length: {}\n\n{}", len, data)

const ICO: &str = include_str!("../bin-up.svg");

pub struct Icon {
    icon: String,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            icon: ICO.to_owned(),
        }
    }
}

impl Icon {
    fn new(req: &Request) -> Result<Self, ErrorStatus> {
        if let Some(query) = req.query()
            && let Some(icon) = query.param("icon")
        {
            return Ok(Self {
                icon: std::fs::read_to_string(&format!("icons/{}.svg", icon))
                    .map_err(|_| err_stt!(404))?,
            });
        };

        Ok(Self::default())
    }
}

impl Resource<Socket> for Icon {
    async fn get(
        &self,
        _socket: &mut Socket,
        _req: Request,
        buf: &mut Vec<u8>,
    ) -> Result<(), ErrorStatus> {
        buf.extend(
            format!(
                "HTTP/1.1 200 Ok\nContent-Type: image/svg+xml\nContent-Length: {}\n\n{}",
                self.icon.len(),
                self.icon
            )
            .as_bytes(),
        );

        Ok(())
    }

    async fn options(
        &self,
        _socket: &mut Socket,
        req: Request,
        buf: &mut Vec<u8>,
    ) -> Result<(), ErrorStatus> {
        buf.extend(b"HTTP/1.1 204 No Content\n");
        Cors::new()
            .origin("127.10.10.1:1024")
            .origin("127.0.0.1:1024")
            .header("*")
            .cors(req.headers(), buf)
            .unwrap();
        buf.push(b'\n');

        Ok(())
    }
}

struct CreateTable;

impl Resource<Socket> for CreateTable {
    async fn put(
        &self,
        socket: &mut Socket,
        _req: Request,
        _buf: &mut Vec<u8>,
    ) -> Result<(), ErrorStatus> {
        let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
";
        socket.conn.execute(query).map_err(|_| err_stt!(500))?;

        Ok(())
    }
}

pub fn lookup(req: &Request, buf: &mut Vec<u8>) -> Result<Services, ErrorStatus> {
    println!("{:?} - {:?}", req.path_str(), req.query());
    Ok(match req.path_str().as_str() {
        "/" => Services::Index(Index::new(req)),
        "/icon" => Services::Icon(Icon::new(req)?),
        "/favicon.ico" => Services::Icon(Icon::default()),
        "/db/create_table" => Services::CreateTable,
        // "/auth/signup"
        _ => {
            not_found(buf);

            Services::Error
        }
    })
}
