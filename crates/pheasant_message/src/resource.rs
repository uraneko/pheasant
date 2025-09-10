use crate::Process;
use pheasant_uri::Route;

struct Resource {
    route: Route,
    redirects: Option<HashSet<Route>>,
    get: Option<Process>,
    post: Option<Process>,
    put: Option<Process>,
    patch: Option<Process>,
    delete: Option<Process>,
    head: bool,
    // WARN this method is a potential security vulnerability
    // at least it may widen bad actors' attack vectors
    trace: bool,
}

impl Resource {
    pub fn builder(route: impl Into<Route>) -> Builder {
        Builder {
            route: route.into(),
            ..Default::default()
        }
    }
}

#[derive(default)]
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
