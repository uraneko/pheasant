use super::{Header, Request, Token};
use crate::{Method, Protocol};
use alloc::vec::Vec;
use pheasant_uri::Resource;

pub fn lex() {}

pub struct Lex<'a> {
    cursor: usize,
    buf: &'a Vec<u8>,
    // eof: bool,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    FailedToParseToken,
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
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn rem(&self) -> usize {
        self.len() - self.cursor
    }
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn buf(&self) -> &[u8] {
        self.buf
    }
}

impl<'a> Lex<'a> {
    pub fn new(buf: &'a mut Vec<u8>) -> Self {
        Self { cursor: 0, buf }
    }

    pub fn method(&mut self) -> Result<Method, Error> {
        let Some(sep) = find(&self.buf, 0, 32) else {
            return Err(Error::CouldntFindWhiteSpace);
        };
        self.cursor = sep + 1;

        Method::try_from(&self.buf[..sep]).map_err(|_| Error::FailedToParseToken)
    }

    pub fn url(&mut self) -> Result<Resource, Error> {
        let Some(sep) = find(&self.buf, self.cursor, 32) else {
            return Err(Error::CouldntFindWhiteSpace);
        };
        let start = self.cursor;
        self.cursor = sep + 1;

        Resource::try_from(&self.buf[start..sep]).map_err(|_| Error::FailedToParseToken)
    }

    pub fn protocol(&mut self) -> Result<(Protocol, Token), Error> {
        let Some((sep, eol)) = find_eol(&self.buf, self.cursor) else {
            return Err(Error::CouldntFindTheEol);
        };
        let start = self.cursor;
        self.cursor = sep + 1;

        Protocol::try_from(&self.buf[start..sep])
            .map_err(|_| Error::FailedToParseToken)
            .map(|p| (p, eol))
    }

    pub fn field(&mut self) -> Result<Token, Error> {
        // TODO
        // if [10, 13].contains(&self.buf[self.cursor + 1]) {
        //     return Err(Error::ArbitraryEol(Token::LF));
        // }
        if let Some(eol) = self.maybe_eol() {
            return Err(Error::ArbitraryEol(eol));
        }

        let Some(sep) = find(&self.buf, self.cursor, b':') else {
            return Err(Error::CouldntFindTheColon);
        };
        let start = self.cursor;
        self.cursor = sep + 1;

        Ok(Token::Field(self.buf[start..sep].to_vec()))
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
        let idx = &mut self.cursor;
        let buf = &self.buf;
        match buf[*idx] {
            10 => {
                if buf[*idx + 1] == 13 {
                    *idx += 2;
                    return Some(Token::LFCR);
                } else {
                    *idx += 1;
                    return Some(Token::LF);
                }
            }
            13 => {
                if buf[*idx + 1] == 10 {
                    *idx += 2;
                    return Some(Token::CRLF);
                } else {
                    return None;
                }
            }
            _ => None,
        }
    }

    pub fn request(&mut self) -> Result<Request, Error> {
        let method = self.method()?;
        let (path, query) = self.url()?.disassemble();
        let (proto, _) = self.protocol()?;
        let headers = self.headers()?;
        let len = content_length(&headers);
        let len = match len {
            // we take from cursor to buffer end
            Err(Error::ContentLengthNotFound) => self.len() - self.cursor,
            Ok(len) => len,
            Err(err) => return Err(err),
        };
        let body = match self.body(len)? {
            Some(Token::Body(body)) => Some(body),
            Some(_) => return Err(Error::UndesirableToken),
            None => None,
        };
        let headers = build_headers(headers)?;

        Ok(Request {
            method,
            path,
            query,
            proto,
            headers,
            body,
        })
    }
}

use std::eprintln;

pub fn build_headers(tokens: Vec<Token>) -> Result<Vec<Header>, Error> {
    let mut iter = tokens.into_iter().rev();
    let mut headers = Vec::new();

    while let Some(token) = iter.next() {
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

    Ok(headers)
}

pub fn content_length(headers: &[Token]) -> Result<usize, Error> {
    if headers
        .iter()
        .filter(|h| {
            let Token::Field(field) = h else { return false };
            field == b"Content-Length"
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
            len == b"Content-Length"
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
