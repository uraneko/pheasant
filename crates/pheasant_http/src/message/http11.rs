use super::Token;
use crate::{Header, Method, Protocol, Status};
use alloc::vec::Vec;
use pheasant_uri::Resource;

pub struct Lex<'a> {
    pub(crate) cursor: usize,
    pub(crate) buf: &'a [u8],
    // eof: bool,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    FailedToParseToken,
    TokenMisbehaves,
    CouldntFindWhiteSpace,
    CouldntFindTheColon,
    CouldntFindTheEol,
    ArbitraryEol(Token),
    ContentLengthOutOfBuffer,
    ContentLengthNotFound,
    MultipleContentLength,
    UndesirableToken,
}

impl<'a> Lex<'a> {
    /// returns length of message buffer
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// returns how much of the message buffer remains
    pub fn rem(&self) -> usize {
        self.len() - self.cursor
    }

    /// returns the cursor position in the message buffer
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// returns a shared ref to the message buffer
    pub fn buf(&self) -> &[u8] {
        self.buf
    }
}

impl<'a> Lex<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { cursor: 0, buf }
    }

    pub fn method(&mut self) -> Result<Method, Error> {
        let Some(sep) = find(&self.buf, 0, 32) else {
            return Err(Error::CouldntFindWhiteSpace);
        };
        let start = self.cursor;
        self.cursor = sep + 1;

        Method::try_from(&self.buf[start..sep]).map_err(|_| Error::FailedToParseToken)
    }

    pub fn status(&mut self) -> Result<Status, Error> {
        let Some(sep) = find(&self.buf, self.cursor, 32) else {
            return Err(Error::CouldntFindWhiteSpace);
        };
        let start = self.cursor;
        self.cursor = sep + 1;

        let code =
            Status::try_from(&self.buf[start..sep]).map_err(|_| Error::FailedToParseToken)?;

        let Some((sep, eol)) = find_eol(&self.buf, self.cursor) else {
            return Err(Error::CouldntFindTheEol);
        };
        let start = self.cursor;
        self.cursor = sep + if Token::LF == eol { 1 } else { 2 };

        let text =
            Status::try_from(&self.buf[start..sep]).map_err(|_| Error::FailedToParseToken)?;

        if code != text {
            return Err(Error::TokenMisbehaves);
        }

        Ok(text)
    }

    pub fn url(&mut self) -> Result<Resource, Error> {
        let Some(sep) = find(&self.buf, self.cursor, 32) else {
            return Err(Error::CouldntFindWhiteSpace);
        };
        let start = self.cursor;
        self.cursor = sep + 1;

        Resource::try_from(&self.buf[start..sep]).map_err(|_| Error::FailedToParseToken)
    }

    pub fn req_proto(&mut self) -> Result<(Protocol, Token), Error> {
        let Some((sep, eol)) = find_eol(&self.buf, self.cursor) else {
            return Err(Error::CouldntFindTheEol);
        };
        let start = self.cursor;
        self.cursor = sep + if Token::LF == eol { 1 } else { 2 };

        Protocol::try_from(&self.buf[start..sep])
            .map_err(|_| Error::FailedToParseToken)
            .map(|p| (p, eol))
    }

    pub fn resp_proto(&mut self) -> Result<Protocol, Error> {
        let Some(sep) = find(&self.buf, self.cursor, 32) else {
            return Err(Error::CouldntFindTheEol);
        };
        let start = self.cursor;
        self.cursor = sep + 1;

        Protocol::try_from(&self.buf[start..sep]).map_err(|_| Error::FailedToParseToken)
    }

    pub fn field(&mut self) -> Result<Token, Error> {
        // TODO
        // if [10, 13].contains(&self.buf[self.cursor + 1]) {
        //     return Err(Error::ArbitraryEol(Token::LF));
        // }
        if let Some(eol) = self.maybe_eol() {
            return Err(Error::ArbitraryEol(eol));
        }

        std::println!("=> {:?}", str::from_utf8(&self.buf[self.cursor..]));
        let Some(sep) = find(&self.buf, self.cursor, b':') else {
            return Err(Error::CouldntFindTheColon);
        };
        let start = self.cursor;
        self.cursor = sep + 1;

        Ok(Token::Field(self.buf[start..sep].to_ascii_lowercase()))
    }

    pub fn value(&mut self) -> Result<[Token; 2], Error> {
        let Some((sep, eol)) = find_eol(&self.buf, self.cursor) else {
            return Err(Error::CouldntFindTheEol);
        };
        let start = sidestep_whitespace(&self.buf, self.cursor);
        self.cursor = sep + if Token::LF == eol { 1 } else { 2 };

        Ok([Token::Value(self.buf[start..sep].to_vec()), eol])
    }

    pub fn header(&mut self) -> Result<[Token; 3], Error> {
        let field = self.field()?;
        let [value, eol] = self.value()?;

        Ok([field, value, eol])
    }

    pub fn headers(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        while self.cursor < self.buf.len() {
            match self.header() {
                Ok(header) => tokens.extend(header),
                Err(Error::ArbitraryEol(tok)) => {
                    tokens.push(tok);
                    return Ok(tokens);
                }
                Err(err) => return Err(err),
            }
        }

        Ok(tokens)
    }

    /// takes only the headers specified by the filters variable
    /// # Example
    /// ```
    /// use pheasant_http::message::http11::{Error, Lex};
    ///
    /// let mut lex = Lex::new(
    ///     b"access-control-request-method: GET\naccess-control-request-header: ranges\norigin: localhost\n");
    ///
    /// let filters: &[&[u8]] = &[
    ///     b"access-control-request-method",
    ///     b"access-control-request-header",
    ///     b"origin"
    /// ];
    /// let cors_headers = lex.headers_filtered(filters)?;
    ///
    /// Ok::<(), Error>(())
    /// ```
    ///
    /// # Errors
    /// - this method could fail when
    /// * the self.field method returns an error
    /// or
    /// * the self.value method returns an error
    ///
    pub fn headers_filtered(&mut self, filters: &[&[u8]]) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        while self.cursor < self.buf.len() {
            match self.field()? {
                Token::Field(field) => {
                    if filters.contains(&field.as_slice()) {
                        let [value, eol] = self.value()?;
                        tokens.extend([Token::Field(field), value, eol]);
                    }
                }
                _ => return Err(Error::UndesirableToken),
            }
        }

        Ok(tokens)
    }

    pub fn body(&mut self, len: usize) -> Result<Option<Token>, Error> {
        if len == 0 {
            return Ok(None);
        }
        // WARN cant naively walk to eof like so buf[cursor..]
        // since that would break features like http1.1 request piping
        let data_end = self.cursor + len;
        // NOTE it's equal when there is only 1 request in the buffer
        if data_end > self.len() {
            return Err(Error::ContentLengthOutOfBuffer);
        }

        Ok(Some(Token::Body(self.buf[self.cursor..data_end].to_vec())))
    }

    pub fn maybe_eol(&mut self) -> Option<Token> {
        let len = self.len();
        let idx = &mut self.cursor;
        let buf = &self.buf;
        match buf[*idx] {
            10 => {
                if *idx + 1 < len && buf[*idx + 1] == 13 {
                    *idx += 2;
                    return Some(Token::LFCR);
                } else {
                    *idx += 1;
                    return Some(Token::LF);
                }
            }
            13 => {
                if *idx + 1 < len && buf[*idx + 1] == 10 {
                    *idx += 2;
                    return Some(Token::CRLF);
                } else {
                    return None;
                }
            }
            _ => None,
        }
    }

    /// this method resets the lexer's cursor to its initial state
    /// i.e., you can then use the method function to get the request method and so on
    pub fn reset(&mut self) {
        self.cursor = 0;
    }
}

// TODO add a feature to arbitrarily walk around (back and forth) the lexer's buffer and get whatever components you want

use std::eprintln;

pub fn build_headers(tokens: Vec<Token>) -> Result<Vec<Header>, Error> {
    let mut iter = tokens.into_iter();
    let mut headers = Vec::new();

    while let Some(token) = iter.next()
        && !token.is_eol()
    {
        let (Token::Field(field), Some(Token::Value(value))) = (token, iter.next()) else {
            return Err(Error::UndesirableToken);
        };

        if iter
            .next()
            .map(|t| !t.is_eol())
            .ok_or(Error::CouldntFindTheEol)?
        {
            return Err(Error::UndesirableToken);
        }

        headers.push(Header::new(field, value));
    }

    // this is never reached since headers always end in a double eol
    Ok(headers)
}

pub fn content_length(headers: &[Token]) -> Result<usize, Error> {
    if headers
        .iter()
        .filter(|h| {
            let Token::Field(field) = h else { return false };
            field == b"content-length"
        })
        .count()
        > 1
    {
        return Err(Error::MultipleContentLength);
    }

    let len_idx = headers
        .iter()
        .position(|t| {
            let Token::Field(len) = t else { return false };
            len == b"content-length"
        })
        .map(|idx| idx + 1);
    let Some(idx) = len_idx else {
        // panic!("Content-Length header not found");
        return Err(Error::ContentLengthNotFound);
    };

    let Ok(len) = ({
        let Token::Value(ref len) = headers[idx] else {
            // panic!("expected content length header value token");
            return Err(Error::UndesirableToken);
        };

        let Ok(s) = str::from_utf8(len) else {
            return Err(Error::FailedToParseToken);
            // panic!("failed to parse content length value into an str");
        };

        s.parse::<usize>()
    }) else {
        return Err(Error::ContentLengthNotFound);
        // panic!("couldn t parse content length header value token");
    };

    Ok(len)
}

fn sidestep_whitespace(buf: &[u8], mut idx: usize) -> usize {
    while buf[idx] == 32 {
        idx += 1;
    }

    idx
}

fn find(buf: &[u8], mut idx: usize, sep: u8) -> Option<usize> {
    while buf[idx] != sep {
        if idx == buf.len() - 1 {
            return None;
        }
        idx += 1;
    }

    Some(idx)
}

fn find_eol(buf: &[u8], mut idx: usize) -> Option<(usize, Token)> {
    let token = loop {
        if idx == buf.len() - 1 {
            if idx == 10 {
                break Token::LF;
            }

            return None;
        }

        if buf[idx] == 13 {
            if buf[idx + 1] == 10 {
                break Token::CRLF;
            }
        } else if buf[idx] == 10 {
            if buf[idx + 1] == 13 {
                break Token::LFCR;
            } else {
                break Token::LF;
            }
        }

        idx += 1;
    };

    Some((idx, token))
}
