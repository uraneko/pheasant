use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    Method(Method),
    Uri(Route),
    Proto(Protocol),
    Header(Vec<u8>),
    Field(Vec<u8>),
    Body(Vec<u8>),
}

fn capitalize_variant(var: &str) -> &str {
    let bytes = var.as_bytes();
    unsafe {
        str::from_utf8_unchecked(
            &[
                [match var.as_bytes[0] {
                    b'm' => b'M',
                    b'u' => b'U',
                    b'p' => b'P',
                    b'h' => b'H',
                    b'f' => b'F',
                    b'b' => b'B',
                }],
                bytes[1..],
            ]
            .concat(),
        )
    }
}

macro_rules! gen_token {
    ($end: expr, $tok: ident) => {
        let var = Ident::new(
            capitalize_variant(stringify!($tok)), Span::call_site());

        macro_rules! #tok {
            ($end: $end) => {
                Token::#var ($end)
            };
        }
    };
}

get_token!(method, uri, proto, header, field, body);

fn lex(inner: &mut [u8], buf: &mut Vec<u8>) -> Vec<Token> {
    // let mut buf = Vec::with_capacity(512);
    let mut reader = BufReader::new(inner);
    let mut toks = Vec::with_capacity(128);

    ReadMethod::method(reader, buf, toks)
        .uri()
        .proto()
        .headers()
        .body()
}

struct ReadMethod;

impl ReadMethod {
    fn method<'a>(
        reader: BufReader<&'a mut [u8]>,
        buf: Vec<u8>,
        mut tokens: Vec<usize>,
    ) -> ReadUri {
        let n = reader.read_until(32, &mut buf).unwrap();
        let method = Method::TryFrom(self.buf.drain(..));
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
    reader: BufReader<&'a mut [u8]>,
    buf: Vec<u8>,
}

impl ReadUri<'_> {
    fn uri(mut self) -> ReadProto {
        let n = self.reader.read_until(32, &mut self.buf).unwrap();
        let uri = Url::try_from(&self.buf);
        self.tokens.push(uri!(uri));
        self.buf.clear();

        ReadProto {
            tokens: self.tokens,
            reader: self.reader,
            buf: self.buf,
        }
    }
}

struct ReadProto<'a> {
    tokens: Vec<Token>,
    reader: BufReader<&'a mut [u8]>,
    buf: Vec<u8>,
}

impl ReadProto<'_> {
    fn proto(mut self) -> ReadHeader {
        let n = self.reader.read_until(10, &mut self.buf).unwrap();
        let proto = Protocol::try_from(self.buf.drain(..)).unwrap();
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
    fn header(buf: &mut Vec<u8>, tokens: &mut Vec<Token>) -> ReadField {
        let parse_len = Self::is_content_length(&buf);

        let Some(idx) = buf.iter().find(|b| **b == b':') else {
            // actually not a header
            unreachable!("expected header line, got something else");
        };
        let bytes = buf.drain(..*idx as usize - 1).collect();
        tokens.push(header!(bytes));

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
    buf: &'a mut Vec<u8>,
    parse_len: bool,
}

impl ReadField {
    fn field(mut self) -> Option<usize> {
        // TODO rfc compliance strictness
        // let respects_cr = self.buf[n - 2] == 13;
        // if mode.is_strict() {
        //     return Err();
        // }
        let bytes = self.buf.drain(..).collect();
        // FIXME this is currently wrong
        // need to use the str_to_num crate functions
        // to convert from the ascii chars to bytes then to usize
        let len = self
            .parse_len
            .then_some(|| usize::from_ne_bytes(bytes.try_into().unwrap()));
        self.tokens.push(field!(bytes));

        len
    }
}

struct ReadHeaders<'a> {
    tokens: Vec<Token>,
    reader: BufReader<&'a mut [u8]>,
    content_length: Option<usize>,
    buf: Vec<u8>,
}

impl ReadHeaders<'_> {
    fn headers(mut self) -> ReadBody {
        while let Ok(n) = self.reader.read_until(10, &mut self.buf) {
            if n == 1 {
                return ReadBody {
                    size: self.content_length,
                    tokens: self.tokens,
                    reader: self.reader,
                    buf: self.buf,
                };
            }

            if let Some(len) = ReadHeader::header(&mut self.buf, &mut self.tokens).field() {
                self.content_length = Some(len);
            }
        }
    }
}

struct ReadBody<'a> {
    size: Option<usize>,
    tokens: Vec<Token>,
    reader: BufReader<&'a mut [u8]>,
    buf: Vec<u8>,
}

impl ReadBody<'_> {
    fn body(mut self) -> Vec<Token> {
        let Some(n) = self.size else {
            return self.tokens;
        };

        self.buf.resize_with(n, Default::default);
        self.reader.read_exact(&mut self.buf);
        self.tokens.push(body!(n));

        self.tokens
    }
}
