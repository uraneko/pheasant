use super::Token;
use crate::{SUB_DELIMS, SpellChecker, SpellingError as Error, UNRESERVED};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct User {
    user: String,
    password: String,
}

impl SpellChecker for User {
    const ALLOWED: &'static [char] = &[':'];
    type Input<'a> = &'a [Token];

    fn spell_check(group: &[Token]) -> Result<(), Error> {
        if group
            .iter()
            .any(|t| !t.is_seq() && !t.is_colon() && !t.is_eql() && !t.is_dot())
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
                    !c.is_alphanumeric()
                        && c != ':'
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

impl User {
    pub fn new(user: String, password: String) -> Self {
        Self { user, password }
    }
}
