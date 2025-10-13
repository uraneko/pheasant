extern crate alloc;
use alloc::string::ToString;
use alloc::{borrow::ToOwned, format, string::String};
use chrono::{DateTime, Utc};
use core::fmt::{self, Debug, Display, Formatter};
use hashbrown::{HashMap, HashSet};
use mime::Mime;

use crate::{FromHeader, IntoHeader};
use pheasant_core::{ErrorStatus, err_stt};

impl IntoHeader<DateTime<Utc>> for String {
    fn into_header(self) -> Result<DateTime<Utc>, ErrorStatus> {
        self.parse::<DateTime<Utc>>()
            .map_err(|_| err_stt!(BadRequest))
    }
}

impl FromHeader<DateTime<Utc>> for String {
    fn from_header(_header: DateTime<Utc>) -> String {
        Utc::now().to_string()
    }
}
