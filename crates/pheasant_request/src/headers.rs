use super::Builder;
use chrono::{DateTime, Utc};
use hashbrown::HashSet;
use mime::Mime;
use pheasant_core::MaybeGlob;
use pheasant_middleware::{FromHeader, FromHeaders, cookies::Cookie, cors::RequestCors};
use pheasant_uri::Origin;
