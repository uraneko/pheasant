extern crate alloc;
use alloc::string::String;

use pheasant_core::{ErrorStatus, err_stt};
use pheasant_uri::Origin;

use crate::IntoHeader;

// host is origin without scheme
// TODO make the host type in uri crate
//
// NOTE this is a client only header
// WARN all http/1.1 requests MUST send a host header
// if no header or more than 1 header is found then the server may return a 400 bad req

impl IntoHeader<Origin> for String {
    fn into_header(self) -> Result<Origin, ErrorStatus> {
        self.parse::<Origin>().map_err(|_| err_stt!(BadStatus))
    }
}
