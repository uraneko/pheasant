pub mod cors;
pub mod errors;
pub mod lookup;
pub mod parse;
pub mod socket;
pub mod stream;

pub use cors::cors;
pub use errors::{bad_request, not_found};
pub use lookup::lookup;
pub use parse::parse;
pub use socket::Socket;
pub use stream::{read_stream, write_stream};

pub trait Service {
    fn run(&self, req: &mut String);
}
