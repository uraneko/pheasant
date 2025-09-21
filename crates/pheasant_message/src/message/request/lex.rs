use crate::io::ReadExt;
use pheasant_core::{Method, Protocol};
use pheasant_uri::Route;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Method(Method),
    Uri(Route),
    Proto(Protocol),
    Header(Vec<u8>),
    Field(Vec<u8>),
    Body(Vec<u8>),
}

macro_rules! gen_token {
     ($($mac: ident / $var: ident),+) => {
         $(
            macro_rules! $mac {
                ($val: expr) => {
                    Token::$var ($val)
                };
            }
         )*
     };
 }

gen_token!(
    method / Method,
    uri / Uri,
    proto / Proto,
    header / Header,
    field / Field,
    body / Body
);

// buf is the socket's buffer
pub fn lex(reader: &[u8], buf: &mut [u8]) -> Vec<Token> {
    // let mut reader = BufReader::new(inner);
    let toks = Vec::with_capacity(128);

    ReadMethod::method(reader, buf, toks)
        .uri()
        .proto()
        .headers()
        .body()
}

struct ReadMethod;

impl ReadMethod {
    fn method<'a>(mut reader: &'a [u8], buf: &'a mut [u8], mut tokens: Vec<Token>) -> ReadUri<'a> {
        let n = reader.read_until(buf, 32).unwrap();
        let method = Method::try_from(&buf[..n]).unwrap();
        tokens.push(method!(method));

        ReadUri {
            tokens,
            reader,
            buf,
        }
    }
}

struct ReadUri<'a> {
    tokens: Vec<Token>,
    reader: &'a [u8],
    buf: &'a mut [u8],
}

impl<'a> ReadUri<'a> {
    fn uri(mut self) -> ReadProto<'a> {
        let n = self.reader.read_until(&mut self.buf, 32).unwrap();
        let uri = Route::try_from(&self.buf[..n]).unwrap();
        self.tokens.push(uri!(uri));

        ReadProto {
            tokens: self.tokens,
            reader: self.reader,
            buf: self.buf,
        }
    }
}

struct ReadProto<'a> {
    tokens: Vec<Token>,
    reader: &'a [u8],
    buf: &'a mut [u8],
}

impl<'a> ReadProto<'a> {
    fn proto(mut self) -> ReadHeaders<'a> {
        let n = self.reader.read_until(&mut self.buf, 10).unwrap();
        let proto = Protocol::try_from(&self.buf[..n]).unwrap();
        self.tokens.push(proto!(proto));

        ReadHeaders {
            tokens: self.tokens,
            reader: self.reader,
            content_length: None,
            buf: self.buf,
        }
    }
}

struct ReadHeader;

impl ReadHeader {
    fn header<'a>(buf: &'a mut [u8], tokens: &'a mut Vec<Token>) -> ReadField<'a> {
        let parse_len = Self::is_content_length(&buf);

        let Some(idx) = buf.iter().find(|b| **b == b':') else {
            // actually not a header
            unreachable!("expected header line, got something else");
        };
        let bytes = &buf[..*idx as usize - 1];
        tokens.push(header!(bytes.to_vec()));

        ReadField {
            tokens,
            buf,
            parse_len,
        }
    }

    fn is_content_length(buf: &[u8]) -> bool {
        buf.starts_with(b"Content-Length")
    }
}

struct ReadField<'a> {
    tokens: &'a mut Vec<Token>,
    buf: &'a mut [u8],
    parse_len: bool,
}

impl ReadField<'_> {
    fn field(self) -> Option<usize> {
        // TODO rfc compliance strictness
        // let respects_cr = self.buf[n - 2] == 13;
        // if mode.is_strict() {
        //     return Err();
        // }
        let bytes = self.buf.to_vec();
        // FIXME this is currently wrong
        // need to use the str_to_num crate functions
        // to convert from the ascii chars to bytes then to usize
        let len = self
            .parse_len
            .then(|| usize::from_ne_bytes(bytes.clone().try_into().unwrap()));
        self.tokens.push(field!(bytes));

        len
    }
}

struct ReadHeaders<'a> {
    tokens: Vec<Token>,
    reader: &'a [u8],
    content_length: Option<usize>,
    buf: &'a mut [u8],
}

impl<'a> ReadHeaders<'a> {
    fn headers(mut self) -> ReadBody<'a> {
        while let Ok(n) = self.reader.read_until(&mut self.buf, 10)
            && n != 1
        {
            if let Some(len) = ReadHeader::header(self.buf, &mut self.tokens).field() {
                self.content_length = Some(len);
            }
        }

        return ReadBody {
            size: self.content_length,
            tokens: self.tokens,
            reader: self.reader,
            buf: self.buf,
        };
    }
}

struct ReadBody<'a> {
    size: Option<usize>,
    tokens: Vec<Token>,
    reader: &'a [u8],
    buf: &'a mut [u8],
}

impl ReadBody<'_> {
    fn body(mut self) -> Vec<Token> {
        let Some(n) = self.size else {
            return self.tokens;
        };

        // self.buf.resize_with(n, Default::default);
        self.reader.read_to(&mut self.buf, n).unwrap();
        self.tokens.push(body!(self.buf[..n].to_vec()));

        self.tokens
    }
}
