extern crate alloc;
use alloc::string::ToString;
use alloc::{borrow::ToOwned, format, string::String};
use chrono::{DateTime, Utc};
use core::fmt::{self, Debug, Display, Formatter};
use hashbrown::{HashMap, HashSet};
use mime::Mime;

use pheasant_core::WildCardish;
use pheasant_uri::Origin;

use crate::{FromHeader, HttpResult, ToHeader};

pub struct Date(DateTime<Utc>);

impl FromHeader for Date {
    fn from_header(header: String) -> HttpResult<Self> {
        Ok(Self(header.parse::<DateTime<Utc>>().unwrap()))
    }
}

pub struct SetDate;

impl ToHeader for SetDate {
    fn to_header(&self) -> String {
        Utc::now().to_string()
    }
}
