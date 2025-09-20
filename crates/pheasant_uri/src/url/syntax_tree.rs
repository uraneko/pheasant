//! the first thing that needs to be done is figuring out the scheme
//! syntax analysis
//! figures out the scheme then
//! validates that the order of components makes sense
//! checks that the component combination is allowed
//! checks that components syntax is valid
use super::lex::Token;
use core::iter::Peekable;
// TODO support semantic urls

// TODO need logic to read the lex tokens and conclude the type of the uri that needs to be parsed
// then generate new syntax tokens by combining the lex tokens

// mod deprecated {
//     use super::*;
//
//     #[derive(Debug, Clone, Copy)]
//     pub enum Error {
//         ExpectedSchemeSepFoundEoT,
//         ExpectedPathOrUserSepFoundEoT,
//         ExpectedUserSepFoundEoT,
//         ExpectedPathSepFoundEoT,
//         ExpectedSlashAfterPortNumber,
//         ExpectedSingleSequenceAsPortToken,
//         ExpectedTokenFoundEoT,
//         ExpectedSlashOrSeqFoundOther,
//         InvalidUriStartingToken,
//         InvalidSepToken,
//         InvalidUriComponentCombination,
//     }
//
//     impl TokenGroup {
//         pub fn is_scheme(&self) -> bool {
//             let Self::Scheme(_) = self else { return false };
//
//             true
//         }
//
//         pub fn is_path(&self) -> bool {
//             let Self::Path(_) = self else { return false };
//
//             true
//         }
//
//         pub fn is_host(&self) -> bool {
//             let Self::Host(_) = self else { return false };
//
//             true
//         }
//
//         pub fn is_query(&self) -> bool {
//             let Self::Query(_) = self else { return false };
//
//             true
//         }
//     }
//
//     fn is_separated(group: &[Token], sep: &[Token]) -> bool {
//         let glen = group.len();
//         let slen = sep.len();
//         glen > slen && &group[glen - slen..] == sep
//     }
//
//     fn clear_separator(group: &mut Vec<Token>, len: usize) {
//         (0..len).into_iter().for_each(|_| {
//             group.pop();
//         });
//     }
//
//     impl TokenGroup {
//         fn scheme<I>(
//             peek: &mut core::iter::Peekable<I>,
//             first: Token,
//             group: &mut Vec<Token>,
//         ) -> Result<TokenGroup, Error>
//         where
//             I: Iterator<Item = Token>,
//         {
//             use Token::*;
//
//             group.push(first);
//             while let Some(token) = peek.next() {
//                 group.push(token);
//
//                 if is_separated(&group, &[Colon, Slash, Slash]) {
//                     clear_separator(group, 3);
//
//                     return Ok(TokenGroup::Scheme(group.drain(..).collect()));
//                 }
//             }
//
//             // expected :// found end of tokens
//             Err(Error::ExpectedSchemeSepFoundEoT)
//         }
//
//         // From https://datatracker.ietf.org/doc/html/rfc3986#page-18
//         // Use of the format "user:password" in the userinfo field is
//         // deprecated.  Applications should not render as clear text any data
//         // after the first colon (":") character found within a userinfo
//         // subcomponent unless the data after the colon is the empty string
//         // (indicating no password).  Applications may choose to ignore or
//         // reject such data when it is received as part of a reference and
//         // should reject the storage of such data in unencrypted form.  The
//         // passing of authentication information in clear text has proven to be
//         // a security risk in almost every case where it has been used.
//         fn user_or_host<I>(
//             peek: &mut core::iter::Peekable<I>,
//             group: &mut Vec<Token>,
//         ) -> Result<TokenGroup, Error>
//         where
//             I: Iterator<Item = Token>,
//         {
//             use Token::*;
//             while let Some(poken) = peek.peek() {
//                 if poken == &Colon {}
//                 if poken == &AddressSign {
//                     // FIXME currently this would break on uri's whose domain doesnt contain dots '.'
//                     // such as http://localhost:3421
//                     return Ok(TokenGroup::User(group.drain(..).collect()));
//                 } else if poken == &Slash {
//                     return Ok(TokenGroup::Host(group.drain(..).collect()));
//                 }
//
//                 let Some(token) = peek.next() else {
//                     unreachable!("peek already gave a pushable token");
//                 };
//                 group.push(token);
//             }
//
//             // expected path sep / or user end @
//             // found end of tokens
//
//             Err(Error::ExpectedPathOrUserSepFoundEoT)
//         }
//
//         fn user<I>(
//             peek: &mut core::iter::Peekable<I>,
//             group: &mut Vec<Token>,
//         ) -> Result<TokenGroup, Error>
//         where
//             I: Iterator<Item = Token>,
//         {
//             use Token::*;
//
//             while let Some(token) = peek.next() {
//                 group.push(token);
//
//                 if is_separated(&group, &[AmperSand]) {
//                     clear_separator(group, 1);
//
//                     return Ok(TokenGroup::User(group.drain(..).collect()));
//                 }
//             }
//
//             // expected user sep @
//             // found end of tokens
//             Err(Error::ExpectedUserSepFoundEoT)
//         }
//
//         fn host<I>(
//             peek: &mut core::iter::Peekable<I>,
//             group: &mut Vec<Token>,
//         ) -> Result<TokenGroup, Error>
//         where
//             I: Iterator<Item = Token>,
//         {
//             use Token::*;
//
//             while let Some(poken) = peek.peek() {
//                 // this way we dont consume the separators
//                 if poken == &Colon || poken == &Slash {
//                     return Ok(TokenGroup::Host(group.drain(..).collect()));
//                 }
//
//                 let Some(token) = peek.next() else {
//                     unreachable!("peek already gave a pushable token");
//                 };
//                 group.push(token);
//             }
//
//             // expected path sep /
//             // found end of tokens
//             Err(Error::ExpectedPathSepFoundEoT)
//         }
//
//         fn port<I>(peek: &mut core::iter::Peekable<I>) -> Result<TokenGroup, Error>
//         where
//             I: Iterator<Item = Token>,
//         {
//             use Token::*;
//
//             if let Some(port @ Seq(_)) = peek.next() {
//                 // what happens next doesnt matter for this token group
//                 // let tok = peek.peek();
//                 // if tok.is_some() && tok != Some(&Slash) {
//                 //     // expected a slash after port number
//                 //     return Err(Error::ExpectedSlashAfterPortNumber);
//                 // }
//
//                 return Ok(TokenGroup::Port(port));
//             }
//
//             // expected a single sequence as port value
//             Err(Error::ExpectedSingleSequenceAsPortToken)
//         }
//
//         fn path<I>(
//             peek: &mut core::iter::Peekable<I>,
//             group: &mut Vec<Token>,
//         ) -> Result<TokenGroup, Error>
//         where
//             I: Iterator<Item = Token>,
//         {
//             use Token::*;
//
//             while let Some(poken) = peek.peek() {
//                 if poken == &QuestionMark || poken == &Pound {
//                     break;
//                 }
//
//                 let Some(token) = peek.next() else {
//                     unreachable!("peek already gave a pushable token");
//                 };
//                 group.push(token);
//             }
//
//             // return the path group
//             // regardless of next == query | frag | none
//             Ok(TokenGroup::Path(group.drain(..).collect()))
//         }
//
//         fn query<I>(
//             peek: &mut core::iter::Peekable<I>,
//             group: &mut Vec<Token>,
//         ) -> Result<TokenGroup, Error>
//         where
//             I: Iterator<Item = Token>,
//         {
//             use Token::*;
//
//             while let Some(poken) = peek.peek() {
//                 if poken == &Pound {
//                     break;
//                 }
//
//                 let Some(token) = peek.next() else {
//                     unreachable!("peek already gave a pushable token")
//                 };
//                 group.push(token);
//             }
//
//             // return query, regardless of wether fragment or end of tokens was encountered
//             Ok(TokenGroup::Query(group.drain(..).collect()))
//         }
//
//         fn fragment<I>(peek: &mut core::iter::Peekable<I>) -> Result<TokenGroup, Error>
//         where
//             I: Iterator<Item = Token>,
//         {
//             return Ok(TokenGroup::Fragment(Vec::from_iter(peek)));
//         }
//     }
//
//     pub fn first_group<I>(
//         peek: &mut core::iter::Peekable<I>,
//         token: Token,
//         group: &mut Vec<Token>,
//     ) -> Result<TokenGroup, Error>
//     where
//         I: Iterator<Item = Token>,
//     {
//         use Token::*;
//         match token {
//             // a scheme
//             Seq(_) => TokenGroup::scheme(peek, token, group),
//             // a path if 1 slash or a domain if 2 slashes
//             Slash => {
//                 match peek.peek() {
//                     // error expected token found end of tokens
//                     None => Err(Error::ExpectedTokenFoundEoT),
//                     // domain start
//                     Some(Slash) => TokenGroup::user_or_host(peek, group),
//                     // path start
//                     Some(Seq(_)) => TokenGroup::path(peek, group),
//                     // error expected slash|seq found other
//                     Some(_) => Err(Error::ExpectedSlashOrSeqFoundOther),
//                 }
//             }
//             // anyting else cant start a url
//             _ => Err(Error::InvalidUriStartingToken),
//         }
//     }
//
//     pub fn component_group<I>(
//         peek: &mut core::iter::Peekable<I>,
//         groups: &mut Vec<TokenGroup>,
//         group: &mut Vec<Token>,
//     ) -> Result<TokenGroup, Error>
//     where
//         I: Iterator<Item = Token>,
//     {
//         let Some(token) = peek.next() else {
//             unreachable!("checked is_some with peekable earlier on");
//         };
//
//         let Some(last) = groups.last() else {
//             return first_group(peek, token, group);
//         };
//
//         use Token::*;
//         use TokenGroup::*;
//         match (token, last) {
//             // a user or host
//             (token, Scheme(_)) => {
//                 group.push(token);
//
//                 TokenGroup::user_or_host(peek, group)
//             }
//             // a host/domain name
//             // must be encountered after a user group
//             (AddressSign, User(_)) => TokenGroup::host(peek, group),
//             // a path
//             (Slash, Host(_) | Port(_)) => TokenGroup::path(peek, group),
//             // a port
//             (Colon, Host(_)) => TokenGroup::port(peek),
//             // a query
//             (QuestionMark, Path(_)) => TokenGroup::query(peek, group),
//             // a fragment
//             (Pound, Path(_) | Query(_)) => TokenGroup::fragment(peek),
//             // invalid separator tokens
//             (Equality | Dot | Seq(_), _) => return Err(Error::InvalidSepToken),
//             // unexpected order/combination of uri components
//             _ => {
//                 return Err(Error::InvalidUriComponentCombination);
//             }
//         }
//     }
//
//     // TODO maybe semantic analysis should be done concurrently with syntatic analysis
//     pub fn component_groups<I>(mut peek: core::iter::Peekable<I>) -> Result<Vec<TokenGroup>, Error>
//     where
//         I: Iterator<Item = Token>,
//     {
//         let mut groups = vec![];
//         let mut group = vec![];
//         while peek.peek().is_some() {
//             let group = component_group(&mut peek, &mut groups, &mut group)?;
//             groups.push(group);
//         }
//
//         Ok(groups)
//     }
//
//     #[deprecated(note = "use syntax_tree instead")]
//     pub fn syntax_tree_deprecated(tokens: Vec<Token>) -> Result<Vec<TokenGroup>, Error> {
//         let peek = tokens.into_iter().peekable();
//
//         component_groups(peek)
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layout {
    Scheme,
    User,
    Host,
    Port,
    Path,
    Query,
    Fragment,
    None,
}

impl Layout {
    fn is_some(&self) -> bool {
        let Self::None = self else { return true };

        false
    }

    fn is_none(&self) -> bool {
        let Self::None = self else { return false };

        true
    }
}

const DEF_LAYOUT: [Layout; 7] = [
    Layout::Scheme,
    Layout::User,
    Layout::Host,
    Layout::Port,
    Layout::Path,
    Layout::Query,
    Layout::Fragment,
];

fn clear_scheme(layout: &mut [Layout; 7]) {
    layout[0] = Layout::None;
}

fn clear_authority(layout: &mut [Layout; 7]) {
    layout[1] = Layout::None;
    layout[2] = Layout::None;
    layout[3] = Layout::None;
}

fn clear_user(layout: &mut [Layout; 7]) {
    layout[1] = Layout::None;
}

fn clear_port(layout: &mut [Layout; 7]) {
    layout[3] = Layout::None;
}

fn clear_query(layout: &mut [Layout; 7]) {
    layout[5] = Layout::None;
}

fn clear_fragment(layout: &mut [Layout; 7]) {
    layout[6] = Layout::None;
}

#[derive(Debug)]
pub enum LayoutError {
    ExpectedSepFoundEoT,
    ExpectedSlashFoundEoT,
    ExpectedAddressOrSeq,
    // expected colon slash or address
    ExpectedEarlySep,
}

const SCHEME_SEP: [Token; 3] = [Token::Colon, Token::Slash, Token::Slash];

// scheme:// | path/bad/me
fn scheme_or_path(tokens: &[Token], layout: &mut [Layout; 7]) -> Result<(), LayoutError> {
    let mut idx = 1;
    while idx < tokens.len() {
        let token = &tokens[idx];
        if !token.is_seq() && !token.is_dot() {
            if idx > 1 && &tokens[idx - 1..=idx] == &[Token::Colon, Token::Slash] {
                idx += 1;
                continue;
            } else if idx > 2
                && &tokens[idx - 2..=idx] == &[Token::Colon, Token::Slash, Token::Slash]
            {
                return Ok(());
            } else if token == &Token::Slash {
                clear_scheme(layout);
                clear_authority(layout);
                return Ok(());
            }
        }

        idx += 1;
    }

    if idx == tokens.len() {
        return Err(LayoutError::ExpectedSepFoundEoT);
    }

    Ok(())
}

// abc:dse@acs.csa:3432 / abc:@iykh.jg:6587 | oioj.hfcg:6578
// WARN i have little confidence in this logic
fn user_or_host(tokens: &[Token], layout: &mut [Layout; 7]) -> Result<(), LayoutError> {
    if !tokens.contains(&Token::AddressSign) {
        clear_user(layout);

        return Ok(());
    }

    let mut idx = 0;
    let mut iter = tokens.iter();
    while let Some(tok) = iter.next() {
        if tok == &Token::Colon {
            match iter.next() {
                None => return Err(LayoutError::ExpectedAddressOrSeq),
                Some(Token::Seq(_)) => match iter.next() {
                    Some(Token::Slash) => return Ok(clear_user(layout)),
                    None => return Err(LayoutError::ExpectedSlashFoundEoT), // error expected slash found eot
                    _ => return Ok(()),
                },
                Some(Token::AddressSign) => return Ok(()),
                _ => continue,
                // _ => return Err(LayoutError::ExpectedAddressOrSeq), // error expected address or seq
            }
        } else if tok == &Token::AddressSign {
            return Ok(());
        } else if tok == &Token::Slash {
            if idx > 2
                && tokens[idx - 1..idx + 1] != SCHEME_SEP
                && tokens[idx - 2..idx] != SCHEME_SEP
            {
                // this is completely messed up since it ignores the existence of scheme
                return Ok(clear_user(layout));
            }
        }
        // doesnt make sense to error out when encountering a token
        // else {
        //     println!("{:?}", tok);
        //     return Err(LayoutError::ExpectedEarlySep); // error expected slash/colon/address
        // }
        idx += 1;
    }

    Ok(())
}

// //auth | /path/to
fn authority_or_path(tokens: &[Token], layout: &mut [Layout; 7]) {
    if tokens[1] != Token::Slash {
        // no authority
        clear_authority(layout);
    }
}

fn maybe_port(tokens: &[Token], layout: &mut [Layout; 7]) {
    if layout[2].is_none() {
        return clear_port(layout);
    }

    let Some(colon) = tokens.iter().position(|t| t == &Token::Colon) else {
        return clear_port(layout);
    };

    if let Some(addr) = tokens.iter().position(|t| t == &Token::AddressSign)
        && addr > colon
    {
        let Some(colon) = tokens[addr + 1..].iter().position(|t| t == &Token::Colon) else {
            return clear_port(layout);
        };

        if let Token::Seq(digits) = &tokens[colon + 1]
            && digits.chars().all(char::is_numeric)
        {
            return;
        }
    } else {
        if let Token::Seq(digits) = &tokens[colon + 1]
            && digits.chars().all(char::is_numeric)
        {
            return;
        }

        clear_port(layout);
    }
}

fn maybe_query(tokens: &[Token], layout: &mut [Layout; 7]) {
    let Some(query) = tokens.iter().position(|t| t == &Token::QuestionMark) else {
        return clear_query(layout);
    };

    let Some(frag) = tokens.iter().position(|t| t == &Token::Pound) else {
        return;
    };

    if frag < query {
        // the ? token exists as part of the fragment
        // and not as the query separator
        return clear_query(layout);
    }
}

fn maybe_fragment(tokens: &[Token], layout: &mut [Layout; 7]) {
    if !tokens.contains(&Token::Pound) {
        clear_fragment(layout);
    }
}

fn layout(tokens: &[Token]) -> Result<[Layout; 7], Error> {
    let mut layout = DEF_LAYOUT;

    // do we have a scheme and/or an authority
    if tokens[0] == Token::Slash {
        // no scheme
        clear_scheme(&mut layout);
        authority_or_path(tokens, &mut layout);
    } else if tokens[0].is_seq() {
        scheme_or_path(tokens, &mut layout)?;
    } else {
        // error invalid first token for url
    }

    // do we still have an authority
    if layout[1].is_some() {
        user_or_host(tokens, &mut layout)?;
    }

    maybe_port(tokens, &mut layout);
    maybe_query(tokens, &mut layout);
    maybe_fragment(tokens, &mut layout);

    Ok(layout)
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

pub struct GroupTokens<I, L>
where
    I: Iterator<Item = Token>,
    L: Iterator<Item = Layout>,
{
    layout: L,
    iter: Peekable<I>,
    groups: Vec<TokenGroup>,
    temp: Vec<Token>,
    with_port: bool,
    with_query: bool,
    with_fragment: bool,
}

#[derive(Debug)]
pub enum Error {
    Layout(LayoutError),
    UnexpectedEoT,
    FoundUnaffiliatedTokens,
    InvalidLastComponent,
    ExpectedSlash,
    ExpectedPortSeq,
}

impl From<LayoutError> for Error {
    fn from(le: LayoutError) -> Self {
        Self::Layout(le)
    }
}

impl<I: Iterator<Item = Token>, L: Iterator<Item = Layout>> GroupTokens<I, L> {
    fn scheme(mut self) -> Result<Self, Error> {
        if self.iter.peek().is_none() {
            return Err(Error::UnexpectedEoT);
        }

        while let Some(token) = self.iter.next() {
            if token == Token::Colon {
                let Some(Token::Slash) = self.iter.next() else {
                    return Err(Error::ExpectedSlash);
                };
                let Some(Token::Slash) = self.iter.next() else {
                    return Err(Error::ExpectedSlash);
                };

                break;
            }

            self.temp.push(token);
        }

        if self.iter.peek().is_none() {
            return Err(Error::InvalidLastComponent);
        }

        self.groups
            .push(TokenGroup::Scheme(self.temp.drain(..).collect()));

        Ok(self)
    }

    fn user(mut self) -> Result<Self, Error> {
        if self.iter.peek().is_none() {
            return Err(Error::UnexpectedEoT);
        }

        while let Some(token) = self.iter.next()
            && token != Token::AddressSign
        {
            self.temp.push(token);
        }

        if self.iter.peek().is_none() {
            return Err(Error::InvalidLastComponent);
        }

        self.groups
            .push(TokenGroup::User(self.temp.drain(..).collect()));

        Ok(self)
    }

    fn host(mut self) -> Result<Self, Error> {
        if self.iter.peek().is_none() {
            return Err(Error::UnexpectedEoT);
        }

        let sep = if self.with_port {
            Token::Colon
        } else {
            Token::Slash
        };

        while let Some(token) = self.iter.next()
            && token != sep
        {
            self.temp.push(token);
        }

        self.groups
            .push(TokenGroup::Host(self.temp.drain(..).collect()));

        Ok(self)
    }

    fn port(mut self) -> Result<Self, Error> {
        if self.iter.peek().is_none() {
            return Err(Error::UnexpectedEoT);
        }

        if let Some(token) = self.iter.next()
            && token.is_seq()
        {
            self.groups.push(TokenGroup::Port(token));
        } else {
            return Err(Error::ExpectedPortSeq);
        }

        Ok(self)
    }

    fn path(mut self) -> Result<Self, Error> {
        if self.iter.peek().is_none() {
            return Err(Error::UnexpectedEoT);
        }

        let sep = match [self.with_query, self.with_fragment] {
            [true, true] => None,
            [false, _] => Some(Token::QuestionMark),
            [true, false] => Some(Token::Pound),
        };

        while let Some(token) = self.iter.next()
            && Some(&token) != sep.as_ref()
        {
            self.temp.push(token);
        }

        self.groups
            .push(TokenGroup::Path(self.temp.drain(..).collect()));

        Ok(self)
    }

    fn query(mut self) -> Result<Self, Error> {
        if self.iter.peek().is_none() {
            return Err(Error::UnexpectedEoT);
        }

        let sep = self.with_fragment.then(|| Token::Pound);
        while let Some(token) = self.iter.next()
            && Some(&token) != sep.as_ref()
        {
            self.temp.push(token);
        }

        self.groups
            .push(TokenGroup::Query(self.temp.drain(..).collect()));

        Ok(self)
    }

    fn fragment(mut self) -> Result<Self, Error> {
        if self.iter.peek().is_none() {
            return Err(Error::UnexpectedEoT);
        }

        while let Some(token) = self.iter.next() {
            self.temp.push(token);
        }

        if self.iter.peek().is_some() {
            return Err(Error::FoundUnaffiliatedTokens);
        }

        self.groups
            .push(TokenGroup::Fragment(self.temp.drain(..).collect()));

        Ok(self)
    }

    fn group(mut self) -> Result<Self, Error> {
        use Layout::*;
        while let Some(l) = self.layout.next() {
            self = match l {
                Scheme => self.scheme()?,
                User => self.user()?,
                Host => self.host()?,
                Port => self.port()?,
                Path => self.path()?,
                Query => self.query()?,
                Fragment => self.fragment()?,
                Layout::None => unreachable!("Layout::None was filtered out in filter_map"),
            };
        }

        Ok(self)
    }

    fn syntax_tree(mut self) -> Result<Vec<TokenGroup>, Error> {
        if self.iter.peek().is_some() {
            return Err(Error::FoundUnaffiliatedTokens);
        }

        Ok(self.groups)
    }
}

pub fn syntax_tree(tokens: Vec<Token>) -> Result<Vec<TokenGroup>, Error> {
    let layout = layout(&tokens)?;
    let [with_port, with_query, with_fragment] = [
        layout[3].is_some(),
        layout[5].is_some(),
        layout[6].is_some(),
    ];
    let layout = layout.into_iter().filter_map(|l| match l {
        Layout::None => None,
        layout => Some(layout),
    });
    let iter = tokens.into_iter().peekable();

    let group_tokens = GroupTokens {
        iter,
        layout,
        with_port,
        with_query,
        with_fragment,
        groups: vec![],
        temp: vec![],
    };

    group_tokens.group()?.syntax_tree()
}
