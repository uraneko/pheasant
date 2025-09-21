use super::Token;
use crate::{SUB_DELIMS, SpellChecker, SpellingError as Error, UNRESERVED};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Path {
    segments: Vec<String>,
}

impl SpellChecker for Path {
    const ALLOWED: &'static [char] = &[':', '@'];
    type Input<'a> = &'a [Token];

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
impl SpellChecker for String {
    const ALLOWED: &'static [char] = &[':', '@', '/', '?'];
    type Input<'a> = &'a [Token];

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

impl Path {
    pub fn from_iter<I: IntoIterator<Item = Token>>(i: I) -> Self {
        let mut iter = i.into_iter().peekable();
        let mut segment = String::new();
        let mut path = Path::default();

        while iter.peek().is_some() {
            path.collect_segment(&mut iter, &mut segment);
        }

        if !segment.is_empty() {
            path.segments.push(segment);
        }

        path
    }

    fn collect_segment(&mut self, iter: &mut impl Iterator<Item = Token>, s: &mut String) {
        while let Some(token) = iter.next() {
            if token == Token::Slash {
                return self.segments.push(s.drain(..).collect());
            }

            s.push_str(token.as_str());
        }
    }
}
