extern crate alloc;
extern crate std;
use hashbrown::HashMap;
use pheasant_core::{Method, Protocol};
use pheasant_uri::Scheme;
use std::io::{Read, Write};

pub mod fallback;
pub mod io;
pub mod message;
pub mod process;
pub mod resource;
pub mod scrutinizer;

pub use fallback::Fallback;
pub use process::Process;
pub use requests::Request;
pub use response::Response;
pub use response_utils::{FindProcess, TakeRequest};

// TODO builder pattern implementations for socket process resource respond and cors
// TODO respond and request
// TODO request headers
// TODO impl Requester Respondent
// TODO socket buffer fill then Request::from_tokens
// TODO Respond Forward Preflight and HttpError logic
// TODO socket event loop
// TODO Scrutinizer impls
// NOTE once all these are done then this pr can be merged
