use super::Token;
use crate::{SUB_DELIMS, SpellChecker, SpellingError as Error, UNRESERVED};

#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct Host {
    labels: Vec<String>,
}

impl SpellChecker for Host {
    const ALLOWED: &'static [char] = &[];
    type Input<'a> = &'a [Token];

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

    pub fn label_too_long(label: &str) -> bool {
        label.len() > Self::LABEL_MAX
    }
}

impl Host {
    pub fn from_iter<I: IntoIterator<Item = Token>>(i: I) -> Self {
        let mut iter = i.into_iter().peekable();
        let mut label = String::new();
        let mut path = Host::default();

        while iter.peek().is_some() {
            path.collect_label(&mut iter, &mut label);
        }

        if !label.is_empty() {
            path.labels.push(label);
        }

        path
    }

    fn collect_label(&mut self, iter: &mut impl Iterator<Item = Token>, s: &mut String) {
        while let Some(token) = iter.next() {
            if token == Token::Dot {
                return self.labels.push(s.drain(..).collect());
            }

            s.push_str(token.as_str());
        }
    }
}

// port
impl SpellChecker for u16 {
    const ALLOWED: &'static [char] = &[];
    type Input<'a> = &'a Token;

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
