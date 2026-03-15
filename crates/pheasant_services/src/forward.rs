use pheasant_prologue::{Status, server::Respond};

pub struct Forward {
    status: Status,
    location: &'static str,
}

impl Forward {
    pub fn new(location: &'static str, status: Status) -> Self {
        Self { status, location }
    }

    /// writes the forward status and location to the given server Respond instance
    pub fn write(self, resp: &mut Respond) {
        resp.status(self.status);
        resp.headers_mut()
            .extend([b"location: ", self.location.as_bytes(), b"\n"].concat());
    }
}

// diff --git a/src/services.rs b/src/services.rs
// index e2cfa27..1c83505 100644
// --- a/src/services.rs
// +++ b/src/services.rs
// @@ -20,6 +20,7 @@ impl Service<Socket> for Services {
//          match self {
//              Self::Auth => Auth::new(&mut req)?.run(socket, req, resp).await,
//              Self::Routing => Routing::new(&req.path_str())?.run(socket, req, resp).await,
// +            Self::Parry => Parry.run(socket, req, resp).await,
//          }
//      }
//  }
// @@ -28,16 +29,34 @@ impl Service<Socket> for Services {
//  pub enum Services {
//      Auth,
//      Routing,
// +    Parry,
//  }
//
//  pub const APP_ROUTES: &[&str] = &["/", "/index.html", "/home", "/auth"];
//
// +pub struct Parry;
// +
// +impl Resource<Socket> for Parry {
// +    async fn get(
// +        self,
// +        _socket: &mut Socket,
// +        _req: Request,
// +        resp: &mut Respond,
// +    ) -> Result<(), pheasant::http::ErrorStatus> {
// +        let frwrd = pheasant::services::Forward::new("/auth", pheasant::http::status!(301));
// +        frwrd.write(resp);
// +
// +        Ok(())
// +    }
// +}
// +
//  pub fn lookup(path: &str) -> Result<Services, ErrorStatus> {
//      Ok(match path {
//          "/auth/remembrance" => Services::Auth,
//          "/auth/field" => Services::Auth,
//          "/auth/cache" => Services::Auth,
//          p if APP_ROUTES.contains(&p) || p.starts_with("/assets/") => Services::Routing,
// +        "/test" => Services::Parry,
//          _ => return err_stt!(?404),
//      })
//  }
//
