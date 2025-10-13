use super::scrutinize::*;
use crate::socket::SocketRef;
use crate::{Request, Scrutinizer};
use pheasant_core::{ErrorStatus, err_stt};

impl Request {
    fn scrutinize(&self, request: &Request, socket: SocketRef<'_>) -> Result<(), ErrorStatus> {
        ScrutinizeSocketSizes::new(request, socket).scrutinize()?;
        ScrutinizeMethod::new(request, socket).scrutinize()?;
        ScrutinizeProto::new(request, socket).scrutinize()?;
        ScrutinizeHeaders::new(request).scrutinize()?;

        Ok(())
    }
}
