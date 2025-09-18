use crate::{Component, SpellChecker};

pub trait PercentEncoded: Component + SpellChecker {
    fn encode(s: &str) -> Result<String, ()>;

    fn decode(s: &str) -> Result<String, ()>;
}
