//! the request is processed once it's raw data is received through a client connection
//!
//! the prerequisite to respond needs 2 inputs: request + resource
//! the condition is
//! req.method == res.method && req.route == res.route -> Respond
//!
//! the prerequisite for a forward is that the response condition fails at route matching
//! the condition is
//! there exists a resource such that res.allows_method(req.method) &&
//! res.redirects.contains(req.route)
//!
//! the prerequisite for a preflight is that req.method == Options
//! the condition is
//! referring to req.requested_method as m; there exists a resource such that res.m is registered
//! and allows cors requests
//!
//! the prerequisite to negotiate is that the request includes the Expect or the Upgrade&Connection headers
//! the condition is
//! 101 -
//! req.headers.contains(Upgrade + Connection) && the server decides to follow through with the
//! upgrade -> we respond with a 101 switching protos
//! 100 -
//! req.headers.contains(Expect = 100-Continue) -> server returns that status code iif it
//! decides to keep the first part of the request and process it
//! 102 -
//! 102 status is deprecated
//! 103 -
//! rarely supported on proto < http2
//! server sends 103 with a Link header to tell the client to preload a resource before the server
//! sends its actual response
//!
//! the prerequisite for an error is that any of the preceeding message variants (req/res/frd/prf)
//! errors out at any point before responding to the client
//! the condition is nothing

extern crate alloc;
extern crate std;

pub mod io;

// TODO builder pattern implementations for socket process resource respond and cors
// TODO respond and request
// TODO request headers
// TODO impl Requester Respondent
// TODO socket buffer fill then Request::from_tokens
// TODO Respond Forward Preflight and HttpError logic
// TODO socket event loop
// TODO Scrutinizer impls
// NOTE once all these are done then this pr can be merged
