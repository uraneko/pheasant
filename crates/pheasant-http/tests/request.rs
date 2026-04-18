use pheasant_http::{
    Method, Protocol,
    message::{
        Token,
        http11::{Error, Lex, build_headers, content_length},
    },
    status,
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
fn req_proto() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();

    assert_eq!(Ok(Protocol::Http11), lexer.req_proto().map(|(p, _)| p));
}

#[test]
fn field() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.req_proto();

    assert_eq!(Ok(Token::Field(b"host".to_vec())), lexer.field());
}

#[test]
fn value() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.req_proto();
    _ = lexer.field();

    assert_eq!(
        Ok(Token::Value(b"example.com".to_vec())),
        lexer.value().map(|[v, _]| v)
    );
}

#[test]
fn end_of_headers() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.req_proto();
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
    _ = lexer.req_proto();
    let tokens = [
        Token::Field(b"host".to_vec()),
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
    _ = lexer.req_proto();
    let tokens = vec![
        Token::Field(b"host".to_vec()),
        Token::Value(b"example.com".to_vec()),
        Token::LFCR,
        Token::Field(b"content-type".to_vec()),
        Token::Value(b"application/x-www-form-urlencoded".to_vec()),
        Token::CRLF,
        Token::Field(b"content-length".to_vec()),
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
    _ = lexer.req_proto();
    let Ok(tokens) = lexer.headers() else {
        panic!("headers are not parsing")
    };

    let Ok(len) = content_length(&tokens) else {
        panic!("couldnt find the content length header");
    };

    assert_eq!(
        Ok(Some(Token::Body(
            b"name=Brian%20Smith&email=brian.smith%40example.com".to_vec()
        ))),
        lexer.body(len)
    );
}

const REQ2: &str = "POST /subscribe?this=one&that=two&like_so HTTP/1.1\nHost: example.com\nContent-Type: application/x-www-form-urlencoded\n\n";

#[test]
fn no_body() {
    let mut buf = REQ2.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);

    let Ok(mut req) = lexer.request() else {
        panic!("request couldnt be lexed properly");
    };
    let req = req.server();

    assert_eq!(req.method(), Method::Post);
    assert_eq!(req.path_str(), "/subscribe".to_owned());

    let Some(query) = req.query() else {
        panic!("expected query to be some value");
    };

    assert_eq!(
        query.params(),
        &hashbrown::HashMap::from([("this".into(), "one".into()), ("that".into(), "two".into())])
    );
    assert_eq!(query.attrs(), &hashbrown::HashSet::from(["like_so".into()]));
    assert_eq!(req.proto(), Protocol::Http11);

    let Ok(headers) = build_headers(vec![
        Token::Field(b"host".to_vec()),
        Token::Value(b"example.com".to_vec()),
        Token::LFCR,
        Token::Field(b"content-type".to_vec()),
        Token::Value(b"application/x-www-form-urlencoded".to_vec()),
        // Token::CRLF,
        // Token::Field(b"content-length".to_vec()),
        // Token::Value(b"50".to_vec()),
        Token::LF,
        Token::LF,
    ]) else {
        panic!("headers couldnt be built");
    };
    assert_eq!(req.headers(), &headers.into());
}

fn request() {
    let mut buf = REQ1.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);

    let Ok(mut req) = lexer.request() else {
        panic!("request couldnt be lexed properly");
    };
    let req = req.server();

    assert_eq!(req.method(), Method::Post);
    assert_eq!(req.path_str(), "/subscribe".to_owned());

    let Some(query) = req.query() else {
        panic!("expected query to be some value");
    };

    assert_eq!(
        query.params(),
        &hashbrown::HashMap::from([("this".into(), "one".into()), ("that".into(), "two".into())])
    );
    assert_eq!(query.attrs(), &hashbrown::HashSet::from(["like_so".into()]));
    assert_eq!(req.proto(), Protocol::Http11);

    let Ok(headers) = build_headers(vec![
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
    ]) else {
        panic!("headers couldnt be built");
    };
    assert_eq!(req.headers(), &headers.into());
    assert_eq!(
        req.body(),
        Some(b"name=Brian%20Smith&email=brian.smith%40example.com".as_slice())
    );
}

#[test]
fn linefeed() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();

    assert_eq!(Ok(Token::LF), lexer.req_proto().map(|(_, sep)| sep));
}

#[test]
fn lf_cr() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.req_proto();
    _ = lexer.field();

    assert_eq!(Ok(Token::LFCR), lexer.value().map(|[_, sep]| sep));
}

#[test]
fn cr_lf() {
    let mut buf = REQ0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.method();
    _ = lexer.url();
    _ = lexer.req_proto();
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
    _ = lexer.req_proto();
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

const RESP0: &str = "HTTP/1.1 200 OK\ncontent-length: 4\ncontent-type:text/plain\r\n\r\nabcd";

#[test]
fn status() {
    let mut buf = RESP0.bytes().collect::<Vec<u8>>();
    let mut lexer = Lex::new(&mut buf);
    _ = lexer.resp_proto();

    assert_eq!(Ok(status!(200)), lexer.status());
}
