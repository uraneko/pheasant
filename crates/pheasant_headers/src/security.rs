// this module is TODO
extern crate alloc;
use alloc::{borrow::ToOwned, format, string::String};
use chrono::{DateTime, Utc};
use core::fmt::{self, Debug, Display, Formatter};
use hashbrown::{HashMap, HashSet};
use mime::Mime;

use pheasant_core::MaybeGlob;
use pheasant_uri::Origin;

use crate::{HttpResult, ToHeader, ToHeaders};

// NOTE this header is server only
pub struct CSP {
    // fetch directives
    child_src: u8,
    connect_src: u8,
    default_src: u8,
    fenced_frame_src: u8,
    font_src: u8,
    frame_src: u8,
    img_src: u8,
    manifest_src: u8,
    media_src: u8,
    object_src: u8,
    prefetch_src: u8,
    script_src: u8,
    script_src_elem: u8,
    script_src_attr: u8,
    style_src: u8,
    style_src_elem: u8,
    style_src_attr: u8,
    worker_src: u8,

    // document directives
    base_uri: u8,
    sandbox: u8,

    // navagation
    form_action: u8,
    frame_ancestors: u8,

    // reporting
    // WARN this directive replaces the report_uri directive
    // for now indicate both in the header value since support is not widespread yet
    report_to: u8,

    // other
    require_trusted_types_for: u8,
    trusted_types: u8,
    upgrade_insecure_request: u8,

    // deprectaed
    #[deprecated]
    report_uri: u8,
    #[deprecated]
    block_all_mixed_content: u8,
}
