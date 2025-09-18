use crate::lex::Token;

#[derive(Debug)]
pub struct Path {
    segments: Vec<String>,
}

impl Path {
    pub fn new(iter: impl Iterator<Item = String>) -> Self {
        Self {
            segments: iter.collect(),
        }
    }
}
