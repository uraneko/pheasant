use super::Resource;
use crate::Servlet;
use hashbrown::HashSet;
use pheasant_core::Redirection;
use pheasant_uri::Route;

#[derive(Default)]
pub struct Builder {
    pub(super) head: bool,
    pub(super) trace: bool,
    pub(super) route: Route,
    pub(super) forwards: Option<HashSet<Route>>,
    pub(super) forward_status: Option<Redirection>,
    pub(super) get: Option<Servlet>,
    pub(super) post: Option<Servlet>,
    pub(super) put: Option<Servlet>,
    pub(super) patch: Option<Servlet>,
    pub(super) delete: Option<Servlet>,
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

    pub fn get(mut self, get: Servlet) -> Self {
        self.get = Some(get);

        self
    }

    pub fn post(mut self, post: Servlet) -> Self {
        self.post = Some(post);

        self
    }

    pub fn put(mut self, put: Servlet) -> Self {
        self.put = Some(put);

        self
    }

    pub fn delete(mut self, delete: Servlet) -> Self {
        self.delete = Some(delete);

        self
    }

    pub fn patch(mut self, patch: Servlet) -> Self {
        self.patch = Some(patch);

        self
    }

    pub fn forward(mut self, redirect: &str) -> Self {
        let redirect: Route = redirect.parse().unwrap();
        let Some(ref mut redirects) = self.forwards else {
            self.forwards = Some(HashSet::from([redirect.into()]));

            return self;
        };

        redirects.insert(redirect);

        self
    }

    pub fn forwards(mut self, r: impl IntoIterator<Item = Route>) -> Self {
        let Some(ref mut redirects) = self.forwards else {
            self.forwards = Some(HashSet::from_iter(r));

            return self;
        };

        redirects.extend(r);

        self
    }

    pub fn forward_status(mut self, s: impl Into<Redirection>) -> Self {
        self.forward_status = Some(s.into());

        self
    }

    pub fn build(self) -> Resource {
        Resource {
            route: self.route,
            forwards: self.forwards,
            forward_status: self.forward_status,
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
