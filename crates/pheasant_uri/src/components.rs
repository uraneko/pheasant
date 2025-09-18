use crate::SpellChecker;

pub mod host;
pub mod path;
pub mod query;
pub mod scheme;
pub mod user;
// URL -> http|https :// user @ host : port / path ? query # fragment

pub use host::Host;
pub use path::Path;
pub use query::Query;
pub use scheme::Scheme;
pub use user::User;
// TODO on http1.1 Host header must always be set once.

pub trait Component {
    const ALLOWED: &[u8];
    // WARN behaviour of this const differns from component to another
    // in host it defines the max number of characters a host can have
    // but in port it defines the highest value that can be assigned to a port number; u16::MAX
    // which is redundant since port is already a u16 value
    const MAX_LEN: usize;

    fn is_forbidden(_ch: u8) -> bool {
        false
    }
}

impl Component for u16 {
    const ALLOWED: &[u8] = &[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
    // TODO probably make this 0
    const MAX_LEN: usize = u16::MAX as usize;

    fn is_forbidden(ch: u8) -> bool {
        ch > b'9' || ch < b'0'
    }
}

impl SpellChecker for u16 {
    fn spell_check(s: &str) -> Result<(), ()> {
        if s.chars().all(|ch| !u16::is_forbidden(ch as u8)) {
            Ok(())
        } else {
            Err(())
        }
    }
}
