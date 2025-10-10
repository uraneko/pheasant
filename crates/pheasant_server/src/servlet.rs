extern crate alloc;
use alloc::boxed::Box;
use alloc::vec;
use core::hash::{Hash, Hasher};
use core::pin::Pin;
use hashbrown::HashSet;
use mime::Mime;
use uuid::Uuid;

use crate::{Request, Respond};
use pheasant_core::{
    ClientError, ErrorStatus, Method, Protocol, Status, Successful, err_stt, status,
};
use pheasant_headers::CorsConfigs;
use pheasant_uri::Route;

pub struct Builder {
    pub(crate) fun: BoxFun,
    pub(crate) status: Status,
    pub(crate) cors: Option<CorsConfigs>,
    pub(crate) mime: Mime,
    pub(crate) query: RequireQuery,
}

impl Builder {
    pub fn cors(mut self, cors: impl Into<CorsConfigs>) -> Self {
        self.cors = Some(cors.into());

        self
    }

    pub fn headers(mut self, headers: String) -> Self {
        self
    }

    pub fn expose(mut self, expose: &str) -> Self {
        self
    }

    pub fn origins(mut self, origin: String) -> Self {
        self
    }

    pub fn credentials(mut self, creds: bool) -> Self {
        self
    }

    pub fn max_age(mut self, max_age: i64) -> Self {
        self
    }

    pub fn mime(mut self, mime: impl Into<Mime>) -> Self {
        self.mime = mime.into();

        self
    }

    pub fn status(mut self, status: impl Into<Status>) -> Self {
        self.status = status.into();

        self
    }

    pub fn query(mut self, query: impl Into<RequireQuery>) -> Self {
        self.query = query.into();

        self
    }

    pub fn build(self) -> Servlet {
        Servlet {
            id: Uuid::new_v4(),
            fun: self.fun,
            status: self.status,
            cors: self.cors,
            mime: self.mime,
            query: self.query,
        }
    }
}

pub struct BuilderCors {
    builder: Builder,
    cors: CorsConfigs,
}

impl Servlet {
    pub fn builder<F, O, R>(fun: F) -> Builder
    where
        F: Fn(R) -> O + Send + Sync + 'static,
        O: Future<Output = Respond> + Send + 'static,
        R: From<Request>,
    {
        Builder {
            fun: Box::new(move |req: Request| {
                let proto = req.proto();

                let input: R = req.into();

                Box::pin(fun(input))
            }),
            cors: None,
            status: status!(200),
            mime: mime::APPLICATION_OCTET_STREAM,
            query: RequireQuery::False,
        }
    }
}

/// a http service type
/// contains the logic that gets executed when a request is made
pub struct Servlet {
    pub(crate) id: uuid::Uuid,
    pub(crate) fun: BoxFun,
    pub(crate) status: Status,
    pub(crate) cors: Option<CorsConfigs>,
    pub(crate) mime: Mime,
    pub(crate) query: RequireQuery,
}

impl Servlet {
    /// processes the request
    pub async fn process(&self, req: Request) -> Respond {
        (self.fun)(req).await
    }

    pub fn is_cross_origin(&self) -> bool {
        self.cors.is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RequireQuery {
    True,
    False,
    Maybe,
}

impl From<bool> for RequireQuery {
    fn from(b: bool) -> Self {
        if b { Self::True } else { Self::False }
    }
}

unsafe impl Send for Servlet {}
unsafe impl Sync for Servlet {}

// the future return type
type BoxFut<'a> = Pin<Box<dyn Future<Output = Respond> + Send + 'a>>;

// the wrapper function type
type BoxFun = Box<dyn Fn(Request) -> BoxFut<'static> + Send + Sync>;

impl Servlet {
    /// creates a new Servlet instance
    ///
    /// you would only use this function directly if you're not using the http method macros
    ///
    /// # Examples
    ///
    /// WARN examples are deprecated
    ///
    /// ```
    /// #[deprecated]
    ///
    /// # fn main() {
    /// let mut phe = Server::new([127, 0, 0, 1], 8883, 3333).unwrap();
    /// phe.service(|| Servlet::new(Method::Get, "/icon", [], "image/svg+xml", svg));
    /// # }
    ///
    /// async fn svg(who: Who) -> Vec<u8> {
    ///     std::fs::read_to_string(who.name).unwrap().into_bytes()
    /// }
    /// ```
    ///
    /// The macro equivalent of the above code would be
    ///
    /// ```
    /// #[deprecated]
    ///
    /// # use crate::Request;
    ///
    /// # fn main() {
    /// let mut phe = Server::new([127, 0, 0, 1], 8883, 3333).unwrap();
    /// phe.service(svg);
    /// # }
    ///
    /// struct StaticFile { path: String }
    ///
    /// impl From<&Request> for StaticFile { ... }
    ///
    /// #[get("/icon")]
    /// #[mime("image/svg+xml")]
    /// async fn svg(file: StaticFile) -> Vec<u8> {
    ///     std::fs::read_to_string(file.path).unwrap().into_bytes()
    /// }
    /// ```
    ///
    pub fn new<F, O, R>(
        mime: Option<Mime>,
        status: Status,
        cors: Option<CorsConfigs>,
        call: F,
        query: RequireQuery,
    ) -> Self
    where
        F: Fn(R, Protocol) -> O + Send + Sync + 'static,
        O: Future<Output = Respond> + Send + 'static,
        R: From<Request>,
    {
        Self {
            id: uuid::Uuid::new_v4(),
            query,
            // method,
            // route,
            mime: mime.unwrap_or_else(|| mime::APPLICATION_OCTET_STREAM),
            status,
            cors,
            // redirects,
            fun: Box::new(move |req: Request| {
                let proto = req.proto();

                let input: R = req.into();

                Box::pin(call(input, proto))
            }),
        }
    }

    // returns a ref to the service logic callback
    pub fn servlet(&self) -> &BoxFun {
        &self.fun
    }

    // pub(crate) fn cors(&self) -> Option<&Cors> {
    //     self.cors.as_ref()
    // }

    /// checks if this service can handle cross origin requests
    pub fn allows_cross_origin_requests(&self) -> bool {
        self.cors.is_some()
    }

    /// if a service supports cors returns &Cors
    ///
    /// else return ErrorStatus
    ///
    /// # Error
    /// 403 Forbidden
    ///
    // # Note
    // - could also use with 401 unauthorized
    //
    // - or 404 not found instead of 403
    // in case server wants to hide the lack of permission from client
    pub fn cors(&self) -> Result<&CorsConfigs, ErrorStatus> {
        self.cors.as_ref().ok_or_else(|| err_stt!(Forbidden))
    }

    // returns a ref to the Mime type if it was provided
    //
    // otherwise returns None
    pub fn clone_mime(&self) -> Mime {
        self.mime.clone()
    }

    pub fn route(&self) -> &str {
        todo!()
    }
}

impl Hash for Servlet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for Servlet {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Servlet {}

pub trait ServletBundle {
    fn iter(self) -> vec::IntoIter<Servlet>;

    fn size(&self) -> usize;
}

impl ServletBundle for Servlet {
    fn iter(self) -> vec::IntoIter<Servlet> {
        vec![self].into_iter()
    }

    fn size(&self) -> usize {
        1
    }
}

impl ServletBundle for [Servlet; 2] {
    fn iter(self) -> vec::IntoIter<Servlet> {
        Vec::from(self).into_iter()
    }

    fn size(&self) -> usize {
        2
    }
}

impl ServletBundle for [Servlet; 3] {
    fn iter(self) -> vec::IntoIter<Servlet> {
        Vec::from(self).into_iter()
    }

    fn size(&self) -> usize {
        3
    }
}

impl ServletBundle for Vec<Servlet> {
    fn iter(self) -> vec::IntoIter<Servlet> {
        self.into_iter()
    }

    fn size(&self) -> usize {
        self.len()
    }
}
