extern crate alloc;
extern crate std;

pub mod fallback;
pub mod io;
pub mod message;
pub mod process;
pub mod resource;
pub mod scrutinizer;

pub use fallback::Fallback;
pub use message::Message;
use message::{ErrorMessage, Forward, Preflight, Request, Respond};
pub use process::Process;
pub use resource::Resource;
use scrutinizer::Scrutinizer;

// TODO builder pattern implementations for socket process resource respond and cors
// TODO respond and request
// TODO request headers
// TODO impl Requester Respondent
// TODO socket buffer fill then Request::from_tokens
// TODO Respond Forward Preflight and HttpError logic
// TODO socket event loop
// TODO Scrutinizer impls
// NOTE once all these are done then this pr can be merged
