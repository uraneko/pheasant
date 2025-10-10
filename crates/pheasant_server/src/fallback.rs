extern crate alloc;
use alloc::boxed::Box;
use core::hash::{Hash, Hasher};
use core::pin::Pin;

use crate::Respond;
use mime::Mime;
use pheasant_core::Status;

pub struct Fallback {
    mime: Option<Mime>,
    status: u16,
    fail: BoxFun,
}

unsafe impl Send for Fallback {}
unsafe impl Sync for Fallback {}

// the future return type
type BoxFut<'a> = Pin<Box<dyn Future<Output = Respond> + Send + 'a>>;

// the wrapper function type
type BoxFun = Box<dyn Fn() -> BoxFut<'static> + Send + Sync>;

impl Fallback {
    pub fn new<'a, F, O>(status: u16, mime: Option<Mime>, fun: F) -> Self
    where
        // probably give the Fn an input of ErrorStatus
        F: Fn() -> O + Send + Sync + 'static,
        O: Future<Output = Respond> + Send + 'static,
    {
        Self {
            status,
            mime,
            fail: Box::new(move || Box::pin(fun())),
        }
    }

    pub fn mime(&self) -> Option<&Mime> {
        self.mime.as_ref()
    }

    pub fn code(&self) -> u16 {
        self.status
    }

    pub fn status(&self) -> Status {
        self.status.try_into().unwrap()
    }

    pub fn fail(&self) -> &BoxFun {
        &self.fail
    }
}

impl Hash for Fallback {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.status.hash(state);
    }
}

impl PartialEq for Fallback {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
    }
}

impl Eq for Fallback {}
