use crate::{Request, Respond, Servlet};
use hashbrown::HashSet;
use pheasant_core::{Method, Redirection};
use pheasant_uri::Route;

pub mod builder;
use builder::Builder;

#[derive(Eq, PartialEq)]
pub struct Resource {
    /// this resource's scheme
    /// no difference between http and https or ws and wss
    // scheme: Scheme,
    /// resource route
    route: Route,
    /// resource redirections
    forwards: Option<HashSet<Route>>,
    /// the resource's forwarding status 3xx
    forward_status: Option<Redirection>,
    /// get method service
    pub get: Option<Servlet>,
    /// post method service
    pub post: Option<Servlet>,
    pub put: Option<Servlet>,
    pub patch: Option<Servlet>,
    pub delete: Option<Servlet>,
    /// allows head method
    pub head: bool,
    // WARN this method is a potential security vulnerability
    // at least it may widen bad actors' attack vectors
    /// allows trace method
    pub trace: bool,
}
// byte_enum_delegate!(Builder, Scheme, schemes, http: Http, https: Https);

impl core::fmt::Debug for Resource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "todo")
    }
}

impl core::hash::Hash for Resource {
    fn hash<S: core::hash::Hasher>(&self, s: &mut S) {}
}

impl Resource {
    pub fn builder(route: &str) -> Builder {
        Builder {
            route: route.parse().unwrap(),
            ..Default::default()
        }
    }
}

macro_rules! method {
    ($self:ident.$method: ident) => {
        match $method {
            Get => $self.get.as_ref(),
            Post => $self.post.as_ref(),
            Put => $self.put.as_ref(),
            Patch => $self.patch.as_ref(),
            Delete => $self.delete.as_ref(),
            _ => None,
        }
    };
    ($self:ident.$method: expr) => {
        match $method {
            Get => $self.get.as_ref(),
            Post => $self.post.as_ref(),
            Put => $self.put.as_ref(),
            Patch => $self.patch.as_ref(),
            Delete => $self.delete.as_ref(),
            _ => None,
        }
    };
}

impl Resource {
    pub fn method_is_cross_origin(&self, method: Method) -> bool {
        use Method::*;

        match method {
            Head => self.head,
            Trace => self.trace,
            Options => true,
            Connect => false,
            _ => method!(self.method)
                .map(|prc| prc.is_cross_origin())
                .unwrap_or_default(),
        }
    }

    pub fn allows_options(&self) -> bool {
        [
            self.get.as_ref(),
            self.post.as_ref(),
            self.put.as_ref(),
            self.patch.as_ref(),
            self.delete.as_ref(),
        ]
        .into_iter()
        .flatten()
        .any(|s| s.cors.is_some())
    }

    // returns an iterator over copies of the resource Methods
    pub fn methods(&self) -> impl Iterator<Item = Method> {
        use Method::*;
        [
            self.get.is_some().then(|| Get),
            self.post.is_some().then(|| Post),
            self.put.is_some().then(|| Put),
            self.patch.is_some().then(|| Patch),
            self.delete.is_some().then(|| Delete),
            self.head.then(|| Head),
            self.trace.then(|| Trace),
            self.allows_options().then(|| Options),
        ]
        .into_iter()
        .flatten()
    }

    // TODO maybe change this to route_str
    /// returns a reference to the String value of the service route
    pub fn route(&self) -> &Route {
        &self.route
    }

    pub fn resource_for(&self, route: &Route, method: Method) -> bool {
        self.route == *route && self[method.as_str()].is_some()
    }

    /// returns the routes that redirect to this service
    /// if any
    pub fn forwards(&self) -> Option<&HashSet<Route>> {
        self.forwards.as_ref()
    }

    // checks if the passed route &str value redirects to this service
    pub fn forwards_to(&self, route: &Route, method: Method) -> bool {
        let Some(ref re) = self.forwards else {
            return false;
        };
        re.iter().find(|r| *r == route).is_some() && self[method.as_str()].is_some()
    }

    pub async fn process(&self, req: Request) -> Respond {
        // here also handle options and forwarding
        self[req.method()].process(req).await
    }
}

impl core::ops::Index<Method> for Resource {
    type Output = Servlet;

    fn index(&self, method: Method) -> &Self::Output {
        match method {
            Method::Get => self.get.as_ref().unwrap(),
            Method::Post => self.post.as_ref().unwrap(),
            Method::Patch => self.patch.as_ref().unwrap(),
            Method::Put => self.put.as_ref().unwrap(),
            Method::Delete => self.delete.as_ref().unwrap(),
            Method::Head => self.get.as_ref().unwrap(),
            _ => panic!("trace doesnt use servlets and connect is a proxy thing"),
        }
    }
}

impl core::ops::Index<&str> for Resource {
    type Output = Option<Servlet>;

    fn index(&self, method: &str) -> &Self::Output {
        match method {
            "GET" => &self.get,
            "POST" => &self.post,
            "PATCH" => &self.patch,
            "PUT" => &self.put,
            "DELETE" => &self.delete,
            "HEAD" => &self.get,
            _ => panic!("trace doesnt use servlets and connect is a proxy thing"),
        }
    }
}
