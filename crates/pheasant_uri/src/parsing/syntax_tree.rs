//! the first thing that needs to be done is figuring out the scheme
//! syntax analysis
//! figures out the scheme then
//! validates that the order of components makes sense
//! checks that the component combination is allowed
//! checks that components syntax is valid
use super::lex::Token;
use crate::{Host, Parse, Path, Query, Scheme};
// TODO support semantic urls

// TODO need logic to read the lex tokens and conclude the type of the uri that needs to be parsed
// then generate new syntax tokens by combining the lex tokens

#[derive(Debug, Clone, Copy)]
pub enum SyntaxError {
    ExpectedSchemeSepFoundEoT,
    ExpectedPathOrUserSepFoundEoT,
    ExpectedUserSepFoundEoT,
    ExpectedPathSepFoundEoT,
    ExpectedSlashAfterPortNumber,
    ExpectedSingleSequenceAsPortToken,
    ExpectedTokenFoundEoT,
    ExpectedSlashOrSeqFoundOther,
    InvalidUriStartingToken,
    InvalidSepToken,
    InvalidUriComponentCombination,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenGroup {
    Scheme(Vec<Token>),
    User(Vec<Token>),
    Host(Vec<Token>),
    Port(Token),
    Path(Vec<Token>),
    Query(Vec<Token>),
    Fragment(Vec<Token>),
}

impl TokenGroup {
    pub fn is_scheme(&self) -> bool {
        let Self::Scheme(_) = self else { return false };

        true
    }

    pub fn is_path(&self) -> bool {
        let Self::Path(_) = self else { return false };

        true
    }

    pub fn is_host(&self) -> bool {
        let Self::Host(_) = self else { return false };

        true
    }
}

fn is_separated(group: &[Token], sep: &[Token]) -> bool {
    let glen = group.len();
    let slen = sep.len();
    glen > slen && &group[glen - slen..] == sep
}

fn clear_separator(group: &mut Vec<Token>, len: usize) {
    (0..len).into_iter().for_each(|_| {
        group.pop();
    });
}

impl TokenGroup {
    fn scheme<I>(
        peek: &mut core::iter::Peekable<I>,
        first: Token,
        group: &mut Vec<Token>,
    ) -> Result<TokenGroup, SyntaxError>
    where
        I: Iterator<Item = Token>,
    {
        use Token::*;

        group.push(first);
        while let Some(token) = peek.next() {
            group.push(token);

            if is_separated(&group, &[Colon, Slash, Slash]) {
                clear_separator(group, 3);

                return Ok(TokenGroup::Scheme(group.drain(..).collect()));
            }
        }

        // expected :// found end of tokens
        Err(SyntaxError::ExpectedSchemeSepFoundEoT)
    }

    fn user_or_host<I>(
        peek: &mut core::iter::Peekable<I>,
        group: &mut Vec<Token>,
    ) -> Result<TokenGroup, SyntaxError>
    where
        I: Iterator<Item = Token>,
    {
        use Token::*;
        while let Some(poken) = peek.peek() {
            if poken == &AddressSign || (poken == &Colon && group.contains(&Dot)) {
                // FIXME currently this would break on uri's whose domain doesnt contain dots '.'
                // such as http://localhost:3421
                // username/password and host/port will need special handling
                // in regards to each other
                return Ok(TokenGroup::User(group.drain(..).collect()));
            } else if poken == &Slash {
                return Ok(TokenGroup::Host(group.drain(..).collect()));
            }

            let Some(token) = peek.next() else {
                unreachable!("peek already gave a pushable token");
            };
            group.push(token);
        }

        // expected path sep / or user end @
        // found end of tokens

        Err(SyntaxError::ExpectedPathOrUserSepFoundEoT)
    }

    fn user<I>(
        peek: &mut core::iter::Peekable<I>,
        group: &mut Vec<Token>,
    ) -> Result<TokenGroup, SyntaxError>
    where
        I: Iterator<Item = Token>,
    {
        use Token::*;

        while let Some(token) = peek.next() {
            group.push(token);

            if is_separated(&group, &[AmperSand]) {
                clear_separator(group, 1);

                return Ok(TokenGroup::User(group.drain(..).collect()));
            }
        }

        // expected user sep @
        // found end of tokens
        Err(SyntaxError::ExpectedUserSepFoundEoT)
    }

    fn host<I>(
        peek: &mut core::iter::Peekable<I>,
        group: &mut Vec<Token>,
    ) -> Result<TokenGroup, SyntaxError>
    where
        I: Iterator<Item = Token>,
    {
        use Token::*;

        while let Some(poken) = peek.peek() {
            // this way we dont consume the separators
            if poken == &Colon || poken == &Slash {
                return Ok(TokenGroup::Host(group.drain(..).collect()));
            }

            let Some(token) = peek.next() else {
                unreachable!("peek already gave a pushable token");
            };
            group.push(token);
        }

        // expected path sep /
        // found end of tokens
        Err(SyntaxError::ExpectedPathSepFoundEoT)
    }

    fn port<I>(peek: &mut core::iter::Peekable<I>) -> Result<TokenGroup, SyntaxError>
    where
        I: Iterator<Item = Token>,
    {
        use Token::*;

        if let Some(port @ Seq(_)) = peek.next() {
            // what happens next doesnt matter for this token group
            // let tok = peek.peek();
            // if tok.is_some() && tok != Some(&Slash) {
            //     // expected a slash after port number
            //     return Err(SyntaxError::ExpectedSlashAfterPortNumber);
            // }

            return Ok(TokenGroup::Port(port));
        }

        // expected a single sequence as port value
        Err(SyntaxError::ExpectedSingleSequenceAsPortToken)
    }

    fn path<I>(
        peek: &mut core::iter::Peekable<I>,
        group: &mut Vec<Token>,
    ) -> Result<TokenGroup, SyntaxError>
    where
        I: Iterator<Item = Token>,
    {
        use Token::*;

        while let Some(poken) = peek.peek() {
            if poken == &QuestionMark || poken == &Pound {
                break;
            }

            let Some(token) = peek.next() else {
                unreachable!("peek already gave a pushable token");
            };
            group.push(token);
        }

        // return the path group
        // regardless of next == query | frag | none
        Ok(TokenGroup::Path(group.drain(..).collect()))
    }

    fn query<I>(
        peek: &mut core::iter::Peekable<I>,
        group: &mut Vec<Token>,
    ) -> Result<TokenGroup, SyntaxError>
    where
        I: Iterator<Item = Token>,
    {
        use Token::*;

        while let Some(poken) = peek.peek() {
            if poken == &Pound {
                break;
            }

            let Some(token) = peek.next() else {
                unreachable!("peek already gave a pushable token")
            };
            group.push(token);
        }

        // return query, regardless of wether fragment or end of tokens was encountered
        Ok(TokenGroup::Query(group.drain(..).collect()))
    }

    fn fragment<I>(peek: &mut core::iter::Peekable<I>) -> Result<TokenGroup, SyntaxError>
    where
        I: Iterator<Item = Token>,
    {
        return Ok(TokenGroup::Fragment(Vec::from_iter(peek)));
    }
}

pub fn first_group<I>(
    peek: &mut core::iter::Peekable<I>,
    token: Token,
    group: &mut Vec<Token>,
) -> Result<TokenGroup, SyntaxError>
where
    I: Iterator<Item = Token>,
{
    use Token::*;
    match token {
        // a scheme
        Seq(_) => TokenGroup::scheme(peek, token, group),
        // a path if 1 slash or a domain if 2 slashes
        Slash => {
            match peek.peek() {
                // error expected token found end of tokens
                None => Err(SyntaxError::ExpectedTokenFoundEoT),
                // domain start
                Some(Slash) => TokenGroup::user_or_host(peek, group),
                // path start
                Some(Seq(_)) => TokenGroup::path(peek, group),
                // error expected slash|seq found other
                Some(_) => Err(SyntaxError::ExpectedSlashOrSeqFoundOther),
            }
        }
        // anyting else cant start a url
        _ => Err(SyntaxError::InvalidUriStartingToken),
    }
}

pub fn component_group<I>(
    peek: &mut core::iter::Peekable<I>,
    groups: &mut Vec<TokenGroup>,
    group: &mut Vec<Token>,
) -> Result<TokenGroup, SyntaxError>
where
    I: Iterator<Item = Token>,
{
    let Some(token) = peek.next() else {
        unreachable!("checked is_some with peekable earlier on");
    };

    let Some(last) = groups.last() else {
        return first_group(peek, token, group);
    };

    use Token::*;
    use TokenGroup::*;
    match (token, last) {
        // a user or host
        (token, Scheme(_)) => {
            group.push(token);

            TokenGroup::user_or_host(peek, group)
        }
        // a host/domain name
        // must be encountered after a user group
        (AddressSign, User(_)) => TokenGroup::host(peek, group),
        // a path
        (Slash, Host(_) | Port(_)) => TokenGroup::path(peek, group),
        // a port
        (Colon, Host(_)) => TokenGroup::port(peek),
        // a query
        (QuestionMark, Path(_)) => TokenGroup::query(peek, group),
        // a fragment
        (Pound, Path(_) | Query(_)) => TokenGroup::fragment(peek),
        // invalid separator tokens
        (Equality | Dot | Seq(_), _) => return Err(SyntaxError::InvalidSepToken),
        // unexpected order/combination of uri components
        t => {
            return Err(SyntaxError::InvalidUriComponentCombination);
        }
    }
}

// TODO maybe semantic analysis should be done concurrently with syntatic analysis
pub fn component_groups<I>(
    mut peek: core::iter::Peekable<I>,
) -> Result<Vec<TokenGroup>, SyntaxError>
where
    I: Iterator<Item = Token>,
{
    let mut groups = vec![];
    let mut group = vec![];
    while peek.peek().is_some() {
        let group = component_group(&mut peek, &mut groups, &mut group)?;
        groups.push(group);
    }

    Ok(groups)
}

pub fn syntax_tree(tokens: Vec<Token>) -> Result<Vec<TokenGroup>, SyntaxError> {
    let peek = tokens.into_iter().peekable();

    component_groups(peek)
}
