use crate::Process;
use hashbrown::HashSet;
use pheasant_core::Method;
use pheasant_uri::Route;

pub struct Resource {
    route: Route,
    redirects: Option<HashSet<Route>>,
    pub get: Option<Process>,
    pub post: Option<Process>,
    pub put: Option<Process>,
    pub patch: Option<Process>,
    pub delete: Option<Process>,
    pub head: bool,
    // WARN this method is a potential security vulnerability
    // at least it may widen bad actors' attack vectors
    pub trace: bool,
}

impl Resource {
    pub fn builder(route: impl Into<Route>) -> Builder {
        Builder {
            route: route.into(),
            ..Default::default()
        }
    }
}

macro_rules! method {
    ($self:ident.$method: ident) => {
        match $method {
            Get => $self.get,
            Post => $self.post,
            Put => $self.put,
            Patch => $self.patch,
            Delete => $self.delete,
            _ => (),
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
                .as_ref()
                .map(|prc| prc.is_cross_origin())
                .unwrap_or_default(),
        }
    }
}

impl Resource {
    pub fn allows_options(&self) -> bool {
        [
            self.get.as_ref(),
            self.post.as_ref(),
            self.put.as_ref(),
            self.patch.as_ref(),
            self.delete.as_ref(),
        ]
        .into_iter()
        .filter(|m| m.is_some())
        .any(|p| p.cors.is_some())
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
        .filter(|m| m.is_some())
        .flatten()
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

#[derive(Default)]
pub struct Builder {
    head: bool,
    trace: bool,
    route: Route,
    redirects: Option<HashSet<Route>>,
    get: Option<Process>,
    post: Option<Process>,
    put: Option<Process>,
    patch: Option<Process>,
    delete: Option<Process>,
}

impl Builder {
    pub fn head(mut self, head: bool) -> Self {
        self.head = head;

        self
    }

    pub fn trace(mut self, trace: bool) -> Self {
        self.trace = trace;
        self
    }

    pub fn get(mut self, get: Process) -> Self {
        self.get = Some(get);

        self
    }

    pub fn post(mut self, post: Process) -> Self {
        self.post = Some(post);

        self
    }

    pub fn put(mut self, put: Process) -> Self {
        self.put = Some(put);

        self
    }

    pub fn delete(mut self, delete: Process) -> Self {
        self.delete = Some(delete);

        self
    }

    pub fn patch(mut self, patch: Process) -> Self {
        self.patch = Some(patch);

        self
    }

    pub fn redirect(mut self, redirect: impl Into<Route>) -> Self {
        let redirect = redirect.into();
        let Some(redirects) = self.redirects else {
            self.redirects = Some(HashSet::from(redirect));
        };

        redirects.insert(redirect);

        self
    }

    pub fn redirects(mut self, redirects: impl IntoIterator<Item = Route>) -> Self {
        let Some(redirects) = self.redirects else {
            self.redirects = Some(HashSet::from_iter(redirects));
        };

        redirects.extend(redirects);

        self
    }

    pub fn build(self) -> Resource {
        Resource {
            route: self.route,
            redirects: self.redirects,
            delete: self.delete,
            patch: self.patch,
            put: self.put,
            post: self.post,
            get: self.get,
            head: self.head,
            trace: self.trace,
        }
    }
}
