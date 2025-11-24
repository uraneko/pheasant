use hashbrown::HashSet;
use pheasant_core::{ErrorStatus, Header, Redirection, err_stt};
use pheasant_uri::Route;

pub struct RouteRequest<'a> {
    request: &'a Route,
    endpoint: &'a Route,
}

impl<'a> RouteRequest<'a> {
    pub fn route(self) -> Result<(), ErrorStatus> {
        if self.request == self.endpoint {
            return Ok(());
        }

        err_stt!(?NotFound)
    }
}

// Resource::route(request, &endpoints) -> Result<Resource, ErrorStatus> {

// }

pub struct ForwardEndpoint<'a> {
    route: &'a Route,
    forwards: &'a HashSet<Route>,
}

impl<'a> ForwardEndpoint<'a> {
    pub fn lookup(route: &Route, forwards: &HashSet<Route>) -> Result<(), ErrorStatus> {
        if forwards.into_iter().any(|r| r == route) {
            return Ok(());
        }

        err_stt!(?NotFound)
    }
}

// Endpoint::builder().forwarding(|| {
//     EndpointForwarder::new(req, ep).lookup()
// })

// respond forwarding
pub struct ForwardRoute<'a> {
    location: &'a Route,
    status: Redirection,
}

impl<'a> ForwardRoute<'a> {
    pub fn forward(self) -> Result<&'static [Header], ErrorStatus> {
        todo!()
    }
}
