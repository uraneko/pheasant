use alloc::boxed::Box;
use alloc::vec;
use core::hash::{Hash, Hasher};
use core::pin::Pin;
use hashbrown::HashSet;
use mime::Mime;
use uuid::Uuid;

use crate::{Request, Response};
use pheasant_core::{ClientError, ErrorStatus, Method, Protocol};
use pheasant_headers::cors::ResourceCors;
use pheasant_uri::Route;

/// a http server service type
/// contains the logic that gets executed when a request is made
// pub struct Service {
//     method: Method,
//     route: Route,
//     redirects: Option<HashSet<Route>>,
//     mime: Option<Mime>,
//     service: BoxFun,
//     // TODO this should become options: ...
//     cors: Option<Cors>,
//     // TODO head: bool,
// }

struct Resource {
    route: Route,
    redirects: Option<HashSet<Route>>,
    get: Option<Service>,
    post: Option<Service>,
    put: Option<Service>,
    patch: Option<Service>,
    delete: Option<Service>,
    head: bool,
    // WARN this method is a potential security vulnerability
    // at least it may widen bad actors' attack vectors
    trace: bool,
}

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

    fn build(self) -> Service {
        Service {
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

impl Service {
    pub fn builder(fun: BoxFun) -> Builder {
        Builder {
            fun,
            cors: None,
            mime: None,
            query: RequireQuery::False,
        }
    }
}

pub struct Service {
    id: uuid::Uuid,
    fun: BoxFun,
    cors: Option<ResourceCors>,
    mime: Option<Mime>,
    query: RequireQuery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RequireQuery {
    True,
    False,
    Maybe,
}

unsafe impl Send for Service {}
unsafe impl Sync for Service {}

// the future return type
type BoxFut<'a> = Pin<Box<dyn Future<Output = Response> + Send + 'a>>;

// the wrapper function type
type BoxFun = Box<dyn Fn(&Request) -> BoxFut<'static> + Send + Sync>;

impl Service {
    /// creates a new Service instance
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
    /// phe.service(|| Service::new(Method::Get, "/icon", [], "image/svg+xml", svg));
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
        O: Future<Output = Response> + Send + 'static,
        R: for<'a> From<&'a Request>,
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

impl Resource {
    // returns a copy of the service Method
    pub fn methods(&self) -> impl Iterator<Item = Method> {
        self.services.keys().map(|m| *m)
    }

    // TODO maybe change this to route_str
    /// returns a reference to the String value of the service route
    pub fn route(&self) -> &str {
        &self.route
    }

    /// returns the routes that redirect to this service
    /// if any
    pub fn re(&self) -> Option<&HashSet<Route>> {
        self.redirects.as_ref()
    }

    // checks if the passed route &str value redirects to this service
    pub(crate) fn redirects_to(&self, route: &str) -> bool {
        let Some(ref re) = self.redirects else {
            return false;
        };
        re.iter().find(|r| r.as_str() == route).is_some()
    }
}

impl Hash for Service {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for Service {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Service {}

pub trait ServiceBundle {
    fn iter(self) -> vec::IntoIter<Service>;

    fn size(&self) -> usize;
}

impl ServiceBundle for Service {
    fn iter(self) -> vec::IntoIter<Service> {
        vec![self].into_iter()
    }

    fn size(&self) -> usize {
        1
    }
}

impl ServiceBundle for [Service; 2] {
    fn iter(self) -> vec::IntoIter<Service> {
        Vec::from(self).into_iter()
    }

    fn size(&self) -> usize {
        2
    }
}

impl ServiceBundle for [Service; 3] {
    fn iter(self) -> vec::IntoIter<Service> {
        Vec::from(self).into_iter()
    }

    fn size(&self) -> usize {
        3
    }
}

impl ServiceBundle for Vec<Service> {
    fn iter(self) -> vec::IntoIter<Service> {
        self.into_iter()
    }

    fn size(&self) -> usize {
        self.len()
    }
}
