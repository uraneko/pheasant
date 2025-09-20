use super::Token;
use crate::{SUB_DELIMS, SpellChecker, SpellingError as Error, UNRESERVED};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Path {
    segments: Vec<String>,
}

impl<'a> SpellChecker for &'a Path {
    const ALLOWED: &'static [char] = &[':', '@'];
    type Input = &'a [Token];

    fn spell_check(group: &[Token]) -> Result<(), Error> {
        if group.iter().any(|t| t.is_qmark() || t.is_pound()) {
            return Err(Error::InvalidTokenForComponent);
        }

        if group
            .iter()
            .filter_map(|t| match t {
                Token::Seq(s) => Some(s),
                _ => None,
            })
            .any(|seq| {
                seq.chars().any(|c| {
                    !c.is_alphanumeric()
                        && !Self::ALLOWED.contains(&c)
                        && !UNRESERVED.contains(&c)
                        && !SUB_DELIMS.contains(&c)
                })
            })
        {
            return Err(Error::InvalidCharsForComponent);
        }

        Ok(())
    }
}

impl Path {
    pub fn new(iter: impl Iterator<Item = String>) -> Self {
        Self {
            segments: iter.collect(),
        }
    }
}

// fragment
impl<'a> SpellChecker for &'a String {
    const ALLOWED: &'static [char] = &[':', '@', '/', '?'];
    type Input = &'a [Token];

    fn spell_check(group: &[Token]) -> Result<(), Error> {
        if group.iter().any(|t| t.is_pound()) {
            return Err(Error::InvalidTokenForComponent);
        }

        if group
            .iter()
            .filter_map(|t| match t {
                Token::Seq(s) => Some(s),
                _ => None,
            })
            .any(|seq| {
                seq.chars().any(|c| {
                    !c.is_alphanumeric()
                        && !Self::ALLOWED.contains(&c)
                        && !UNRESERVED.contains(&c)
                        && !SUB_DELIMS.contains(&c)
                })
            })
        {
            return Err(Error::InvalidCharsForComponent);
        }

        Ok(())
    }
}
