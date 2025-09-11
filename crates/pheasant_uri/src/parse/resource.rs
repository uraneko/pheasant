use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, SerializeTupleStruct, Serializer};
use std::collections::{HashMap, HashSet};

use super::TransmuteError;
use super::route::Route;
use crate::{Query, Url};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Resource {
    route: Route,
    query: Option<Query>,
}

impl Resource {
    pub fn from_parts(route: Route, query: Option<Query>) -> Self {
        Self { route, query }
    }
}

impl std::str::FromStr for Resource {
    type Err = TransmuteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Url>().unwrap().interpret::<Self>()
    }
}

impl From<Resource> for (Route, Query) {
    fn from(rsrc: Resource) -> (Route, Query) {
        (rsrc.route, rsrc.query)
    }
}

impl Resource {
    pub fn query(&self) -> Option<&Query> {
        let Some(ref query) = self.query else {
            return None;
        };

        Some(query)
    }

    pub fn contains_query(&self) -> bool {
        self.query.is_some()
    }

    pub fn params(&self) -> Option<&HashMap<String, String>> {
        let Some(ref query) = self.query else {
            return None;
        };

        Some(query.params())
    }

    pub fn attrs(&self) -> Option<&HashSet<String>> {
        let Some(ref query) = self.query else {
            return None;
        };

        Some(query.attrs())
    }

    pub fn contains_param(&self, k: &str) -> bool {
        let Some(params) = self.params() else {
            return false;
        };

        params.contains_key(k)
    }

    pub fn contains_attr(&self, k: &str) -> bool {
        let Some(attrs) = self.attrs() else {
            return false;
        };

        attrs.contains(k)
    }

    /// takes route from self
    pub fn take_route(&mut self) -> Route {
        std::mem::take(&mut self.route)
    }

    /// takes query from self
    pub fn take_query(&mut self) -> Option<Query> {
        std::mem::take(&mut self.query)
    }

    pub fn sequence(&self) -> String {
        let Some(ref query) = self.query else {
            return self.route.as_str().to_owned();
        };

        let mut seq = query.sequence();
        seq.insert_str(0, self.route.as_str());

        seq
    }
}

impl TryFrom<Url> for Resource {
    type Error = TransmuteError;

    fn try_from(mut url: Url) -> Result<Self, Self::Error> {
        let Some(path) = url.take_path() else {
            return Err(TransmuteError::RoutePathNotFound);
        };

        Ok(Self {
            route: Route::new(path),
            query: url.take_query(),
        })
    }
}

impl From<&Route> for TokenTree {
    fn from(route: &Route) -> Self {
        let mut ts = TS2::new();
        let ident = Ident::new("Route", Span::call_site());
        ts.append(ident);

        let lit = Group::new(
            Delimiter::Parenthesis,
            TokenTree::Literal(Literal::string(route.as_str())).into(),
        );
        ts.append(lit);

        let group = Group::new(Delimiter::None, ts);
        TokenTree::from(group)
    }
}
