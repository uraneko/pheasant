pub mod lex;
pub mod read;
pub mod scrutinize;

pub use lex::{Token, lex};

use crate::socket::SocketRef;
use crate::{Request, Scrutinizer};
use pheasant_core::{ErrorStatus, err_stt};
use scrutinize::*;

impl Request {
    pub fn scrutinize(&self, socket: SocketRef<'_>) -> Result<(), ErrorStatus> {
        ScrutinizeSocketSizes::new(self, socket).scrutinize()?;
        ScrutinizeMethod::new(self, socket).scrutinize()?;
        ScrutinizeProto::new(self, socket).scrutinize()?;
        ScrutinizeHeaders::new(self).scrutinize()?;

        Ok(())
    }
}
