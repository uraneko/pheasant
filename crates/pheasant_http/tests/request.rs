use pheasant_http::{
    Method, Protocol,
    request::{
        Token,
        http11::{Error, Lex},
    },
};

const REQ0: &str = "POST /subscribe HTTP/1.1\nHost: example.com\n\rContent-Type: application/x-www-form-urlencoded\r\nContent-Length: 50\n\nname=Brian%20Smith&email=brian.smith%40example.com";

#[test]
fn method() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);

    assert_eq!(Ok(Method::Post), lexer.method());
}

#[test]
fn path() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    let Ok(resource) = lexer.url() else {
        panic!("resource is err");
    };

    assert_eq!(&resource.path(), "/subscribe");
    assert!(resource.query().is_none());
}

const REQ1: &str = "POST /subscribe?this=one&that=two&like_so HTTP/1.1\nHost: example.com\nContent-Type: application/x-www-form-urlencoded\nContent-Length: 50\n\nname=Brian%20Smith&email=brian.smith%40example.com";

#[test]
fn path_query() {
    let mut buf = REQ1.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    let Ok(resource) = lexer.url() else {
        panic!("resource is err");
    };

    let Some(query) = resource.query() else {
        panic!("query exists");
    };

    assert_eq!(
        query.params(),
        &hashbrown::HashMap::from([("this".into(), "one".into()), ("that".into(), "two".into())])
    );

    assert_eq!(query.attrs(), &hashbrown::HashSet::from(["like_so".into()]));
}

#[test]
fn protocol() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();

    assert_eq!(Ok(Protocol::Http11), lexer.protocol().map(|(p, _)| p));
}

#[test]
fn field() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.protocol();

    assert_eq!(Ok(Token::Field(b"Host".to_vec())), lexer.field());
}

#[test]
fn value() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.protocol();
    _ = lexer.field();

    assert_eq!(
        Ok(Token::Value(b"example.com".to_vec())),
        lexer.value().map(|[v, _]| v)
    );
}

#[test]
fn last_field() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.protocol();
    _ = lexer.field();
    _ = lexer.value();
    _ = lexer.field();
    _ = lexer.value();
    _ = lexer.field();
    _ = lexer.value();

    let cursor = lexer.cursor();
    let Err(Error::ArbitraryEol(Token::LF)) = lexer.field() else {
        panic!("expected an lf eol error")
    };

    assert_eq!(cursor + 1, lexer.cursor());
}

#[test]
fn header() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.protocol();
    let tokens = [
        Token::Field(b"Host".to_vec()),
        Token::Value(b"example.com".to_vec()),
        Token::LFCR,
    ];

    assert_eq!(Ok(tokens), lexer.header())
}

#[test]
fn headers() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.protocol();
    let tokens = vec![
        Token::Field(b"Host".to_vec()),
        Token::Value(b"example.com".to_vec()),
        Token::LFCR,
        Token::Field(b"Content-Type".to_vec()),
        Token::Value(b"application/x-www-form-urlencoded".to_vec()),
        Token::CRLF,
        Token::Field(b"Content-Length".to_vec()),
        Token::Value(b"50".to_vec()),
        Token::LF,
        Token::LF,
    ];

    assert_eq!(Ok(tokens), lexer.headers());
}

#[test]
fn body() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.protocol();
    let Ok(tokens) = lexer.headers() else {
        panic!("headers are not parsing")
    };
    let len_idx = tokens
        .iter()
        .position(|t| {
            let Token::Field(len) = t else { return false };
            len == b"Content-Length"
        })
        .map(|idx| idx + 1);
    let Some(idx) = len_idx else {
        panic!("Content-Length header not found");
    };

    let Ok(len) = ({
        let Token::Value(ref len) = tokens[idx] else {
            panic!("expected content length header value token");
        };

        let Ok(s) = str::from_utf8(len) else {
            panic!("failed to parse content length value into an str");
        };

        s.parse::<usize>()
    }) else {
        panic!("couldn t parse content length header value token");
    };

    assert_eq!(
        Ok(Token::Body(
            b"name=Brian%20Smith&email=brian.smith%40example.com".to_vec()
        )),
        lexer.body(len)
    );
}

#[test]
fn linefeed() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();

    assert_eq!(Ok(Token::LF), lexer.protocol().map(|(_, sep)| sep));
}

#[test]
fn lf_cr() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.protocol();
    _ = lexer.field();

    assert_eq!(Ok(Token::LFCR), lexer.value().map(|[_, sep]| sep));
}

#[test]
fn cr_lf() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.protocol();
    _ = lexer.field();
    _ = lexer.value();
    _ = lexer.field();

    assert_eq!(Ok(Token::CRLF), lexer.value().map(|[_, sep]| sep));
}

#[test]
fn maybe_eol() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.protocol();
    _ = lexer.field();
    _ = lexer.value();
    _ = lexer.field();
    _ = lexer.value();
    _ = lexer.field();
    _ = lexer.value();

    let cursor = lexer.cursor();
    assert_eq!(Some(Token::LF), lexer.maybe_eol());
    assert_eq!(cursor + 1, lexer.cursor());
    assert!(lexer.maybe_eol().is_none());
    assert_eq!(cursor + 1, lexer.cursor());
}
