use super::Token;
use crate::{SUB_DELIMS, SpellChecker, SpellingError as Error, UNRESERVED};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Host {
    labels: Vec<String>,
}

impl<'a> SpellChecker for &'a Host {
    const ALLOWED: &'static [char] = &[];
    type Input = &'a [Token];

    fn spell_check(group: &[Token]) -> Result<(), Error> {
        if group
            .iter()
            .any(|t| !t.is_seq() && !t.is_dot() && !t.is_eql())
        {
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
                    !c.is_alphanumeric() && !UNRESERVED.contains(&c) && !SUB_DELIMS.contains(&c)
                })
            })
        {
            return Err(Error::InvalidCharsForComponent);
        }

        Ok(())
    }
}

impl Host {
    // single label max length
    const LABEL_MAX: usize = 63;
    // all labels length sum
    const HOST_MAX: usize = 255;

    pub fn new(labels: impl Iterator<Item = String>) -> Self {
        Self {
            labels: labels.collect(),
        }
    }

    fn label_too_long(label: &str) -> bool {
        label.len() > Self::LABEL_MAX
    }
}

// port
impl<'a> SpellChecker for &'a u16 {
    const ALLOWED: &'static [char] = &[];
    type Input = &'a Token;

    fn spell_check(token: &Token) -> Result<(), Error> {
        let Token::Seq(seq) = token else {
            return Err(Error::InvalidTokenForComponent);
        };

        if seq.chars().any(|c| !c.is_numeric()) {
            return Err(Error::InvalidCharsForComponent);
        }

        Ok(())
    }
}
