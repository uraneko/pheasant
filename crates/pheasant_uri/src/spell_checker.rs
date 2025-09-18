use crate::Component;

pub enum SyntaxError {}

pub trait SpellChecker: Component {
    fn spell_check(s: &str) -> Result<(), ()>;
}
