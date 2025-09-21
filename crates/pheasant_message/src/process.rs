use alloc::boxed::Box;
use alloc::vec;
use core::hash::{Hash, Hasher};
use core::pin::Pin;
use hashbrown::HashSet;
use mime::Mime;
use uuid::Uuid;

use crate::{Request, Respond};
use pheasant_core::{ClientError, ErrorStatus, Method, Protocol};
use pheasant_headers::cors::ResourceCors;
use pheasant_uri::Route;

/// a http server service type
/// contains the logic that gets executed when a request is made
// pub struct Process {
//     method: Method,
//     route: Route,
//     redirects: Option<HashSet<Route>>,
//     mime: Option<Mime>,
//     service: BoxFun,
//     // TODO this should become options: ...
//     cors: Option<Cors>,
//     // TODO head: bool,
// }

pub struct Builder {
    fun: BoxFun,
    cors: Option<ResourceCors>,
    mime: Option<Mime>,
    query: RequireQuery,
}

impl Builder {
    pub fn cors(mut self, cors: impl Into<ResourceCors>) -> Self {
        self.cors = Some(cors.into());

        self
    }

    pub fn mime(mut self, mime: impl Into<Mime>) -> Self {
        self.mime = Some(mime.into());

        self
    }

    pub fn query(mut self, query: impl Into<RequireQuery>) -> Self {
        self.query = query.into();

        self
    }

    fn build(self) -> Process {
        Process {
            id: Uuid::new_v4(),
            fun: self.fun,
            cors: self.cors,
            mime: self.mime,
            query: self.query,
        }
    }
}

pub struct BuilderCors {
    builder: Builder,
    cors: ResourceCors,
}

impl Process {
    pub fn builder(fun: BoxFun) -> Builder {
        Builder {
            fun,
            cors: None,
            mime: None,
            query: RequireQuery::False,
        }
    }
}

pub struct Process {
    id: uuid::Uuid,
    fun: BoxFun,
    cors: Option<ResourceCors>,
    mime: Option<Mime>,
    query: RequireQuery,
}

impl Process {
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

unsafe impl Send for Process {}
unsafe impl Sync for Process {}

// the future return type
type BoxFut<'a> = Pin<Box<dyn Future<Output = Respond<'a>> + Send + 'a>>;

// the wrapper function type
type BoxFun = Box<dyn Fn(&Request) -> BoxFut<'static> + Send + Sync>;

impl Process {
    /// creates a new Process instance
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
    /// phe.service(|| Process::new(Method::Get, "/icon", [], "image/svg+xml", svg));
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
    pub fn new<'a, F, O, R>(
        method: Method,
        route: Route,
        redirects: Option<HashSet<Route>>,
        mime: Option<Mime>,
        cors: Option<ResourceCors>,
        call: F,
        query: RequireQuery,
    ) -> Self
    where
        F: Fn(R, Protocol) -> O + Send + Sync + 'static,
        O: Future<Output = Respond<'a>> + Send + 'static,
        R: for<'b> From<&'b Request>,
    {
        Self {
            id: uuid::Uuid::new_v4(),
            query,
            // method,
            // route,
            mime,
            cors,
            // redirects,
            fun: Box::new(move |req: &Request| {
                let proto = req.proto();

                let input: R = req.into();

                Box::pin(call(input, proto))
            }),
        }
    }

    // returns a ref to the service logic callback
    pub fn service(&self) -> &BoxFun {
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
    /// else return StatusError
    ///
    /// # Error
    /// 403 Forbidden
    ///
    // # Note
    // - could also use with 401 unauthorized
    //
    // - or 404 not found instead of 403
    // in case server wants to hide the lack of permission from client
    pub fn cors(&self) -> Result<&ResourceCors, ErrorStatus> {
        self.cors
            .as_ref()
            .ok_or_else(|| ErrorStatus::Client(ClientError::Forbidden))
    }

    // returns a ref to the Mime type if it was provided
    //
    // otherwise returns None
    pub fn clone_mime(&self) -> Option<Mime> {
        self.mime.clone()
    }

    pub fn route(&self) -> &str {
        todo!()
    }
}

impl Hash for Process {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Process {}

pub trait ProcessBundle {
    fn iter(self) -> vec::IntoIter<Process>;

    fn size(&self) -> usize;
}

impl ProcessBundle for Process {
    fn iter(self) -> vec::IntoIter<Process> {
        vec![self].into_iter()
    }

    fn size(&self) -> usize {
        1
    }
}

impl ProcessBundle for [Process; 2] {
    fn iter(self) -> vec::IntoIter<Process> {
        Vec::from(self).into_iter()
    }

    fn size(&self) -> usize {
        2
    }
}

impl ProcessBundle for [Process; 3] {
    fn iter(self) -> vec::IntoIter<Process> {
        Vec::from(self).into_iter()
    }

    fn size(&self) -> usize {
        3
    }
}

impl ProcessBundle for Vec<Process> {
    fn iter(self) -> vec::IntoIter<Process> {
        self.into_iter()
    }

    fn size(&self) -> usize {
        self.len()
    }
}
