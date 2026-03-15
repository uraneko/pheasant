use pheasant::http::{ErrorStatus, Respond, err_stt, request::Request, status};
use pheasant::services::{Cors, Resource, Service, Socket, not_found};

pub enum Services {
    Index(Index),
    Icon(Icon),
    CreateTable,
    Error,
}

impl Service<Socket> for Services {
    async fn serve(
        &self,
        socket: &mut Socket,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        match self {
            Self::Index(index) => index.run(socket, req, resp).await,
            Self::Icon(icon) => icon.run(socket, req, resp).await,
            Self::CreateTable => CreateTable.run(socket, req, resp).await,
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
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        let html0 = b"<!DOCTYPE html><html><body style='background:";
        let html1 = b";color:beige'><h1>hello web</h1></body></html>";
        let len = html0.len() + html1.len() + self.background.len();
        resp.headers_mut()
            .extend(format!("Content-Type:text/html\nContent-Length:{}\n", len,).as_bytes());
        resp.body_mut().extend(html0);
        resp.body_mut().extend(self.background.as_bytes());
        resp.body_mut().extend(html1);

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
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        resp.headers_mut().extend(
            format!(
                "Content-Type: image/svg+xml\nContent-Length: {}\n",
                self.icon.len(),
            )
            .as_bytes(),
        );
        resp.body_mut().extend(self.icon.as_bytes());

        Ok(())
    }

    async fn options(
        &self,
        _socket: &mut Socket,
        req: Request,
        resp: &mut Respond,
    ) -> Result<(), ErrorStatus> {
        resp.status(status!(204));
        Cors::new()
            .origin("127.10.10.1:1024")
            .origin("127.0.0.1:1024")
            .header("*")
            .cors(req.headers(), resp.headers_mut())
            .unwrap();

        Ok(())
    }
}

struct CreateTable;

impl Resource<Socket> for CreateTable {
    async fn put(
        &self,
        socket: &mut Socket,
        _req: Request,
        _resp: &mut Respond,
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

pub fn lookup(req: &Request, resp: &mut Respond) -> Result<Services, ErrorStatus> {
    // println!("{:?} - {:?}", req.path_str(), req.query());
    Ok(match req.path_str().as_str() {
        "/" => Services::Index(Index::new(req)),
        "/icon" => Services::Icon(Icon::new(req)?),
        "/favicon.ico" => Services::Icon(Icon::default()),
        "/db/create_table" => Services::CreateTable,
        // "/auth/signup"
        _ => {
            not_found(resp);

            Services::Error
        }
    })
}
