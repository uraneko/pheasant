use embedded_io::Write;
use mime::{FromStrError, Mime};

pub struct Content {
    ty: Mime,
    len: usize,
    // language: Option<Language>,
}

impl Content {
    pub fn with_len(len: usize) -> Self {
        Self {
            len,
            ty: mime::APPLICATION_OCTET_STREAM,
        }
    }

    pub fn from_params(len: usize, ty: &str) -> Result<Self, FromStrError> {
        ty.parse().map(|ty| Self { len, ty })
    }

    pub fn new(data: &[u8]) -> Self {
        Self {
            ty: guess_mime(data),
            len: data.len(),
        }
    }

    pub fn guess_mime(mut self, data: &[u8]) -> Self {
        self.ty = guess_mime(data);

        self
    }

    pub fn mime_from_ext(mut self, ext: &str) -> Self {
        self.ty = mime_from_ext(ext);

        self
    }

    pub fn force_mime(mut self, ty: &str) -> Result<Self, FromStrError> {
        self.ty = ty.parse()?;

        Ok(self)
    }

    pub fn dump_headers<W>(self, headers: &mut W) -> Result<(), W::Error>
    where
        W: Write,
    {
        headers.write(b"content-length: ")?;
        headers.write(self.len.to_string().as_bytes())?;
        headers.write(&[10])?;
        headers.write(b"content-type: ")?;
        headers.write(self.ty.essence_str().as_bytes())?;
        headers.write(&[10])?;

        Ok(())
    }

    // fn with_lang(data: &[u8], lang: impl Into<Language>) -> Self {
    //     Self {
    //         type_: mime::TEXT_PLAIN,
    //         length: data.len(),
    //         language: Some(lang.into()),
    //     }
    // }

    // fn language(&mut self, lang: impl Into<Language>) {
    //     self.language = Some(lang.into());
    // }
}

pub fn mime_from_ext(ext: &str) -> Mime {
    match ext {
        "html" => mime::TEXT_HTML,
        "js" => mime::TEXT_JAVASCRIPT,
        "json" => mime::APPLICATION_JSON,
        "css" => mime::TEXT_CSS,
        "svg" => mime::IMAGE_SVG,
        _ => mime::TEXT_PLAIN,
    }
}

pub fn guess_mime(data: &[u8]) -> Mime {
    let Ok(s) = str::from_utf8(data) else {
        return mime::APPLICATION_OCTET_STREAM;
    };

    match s.trim() {
        s if s.starts_with('{') && s.ends_with('}') || s.starts_with('[') && s.ends_with(']') => {
            mime::APPLICATION_JSON
        }
        s if s.starts_with("<!DOCTYPE html>") => mime::TEXT_HTML,
        s if probably_css(s) => mime::TEXT_CSS,
        s if s.starts_with("<?xml ") && s.ends_with("</svg>") => mime::IMAGE_SVG,
        s if probably_rust(s) => mime::TEXT_PLAIN,
        s if probably_js(s) => mime::TEXT_JAVASCRIPT,
        _ => mime::TEXT_PLAIN,
    }
}

const CSS_SYNTAX: &[&str] = &[
    "width:",
    "height:",
    "display: flex",
    "font-",
    ":is(",
    ":hover",
    ":focus",
    ":not(",
    "display: grid",
];

fn probably_css(s: &str) -> bool {
    CSS_SYNTAX.iter().any(|expr| s.contains(expr))
}

const JS_SYNTAX: &[&str] = &[
    ") => {",
    "function ",
    " = new ",
    "class ",
    " extends ",
    "startsWith",
    "indexOf",
    "parentElement",
    "querySelector",
    "getElement",
    "document.",
    "addEventListener",
    "new MutationObserver",
    "childNodes",
    "children",
    ".styles",
    "Attribute(",
    "Property(",
];

fn probably_js(s: &str) -> bool {
    JS_SYNTAX.iter().any(|expr| s.contains(expr))
}

const RUST_SYNTAX: &[&str] = &[
    "fn main(",
    " fn ",
    "(|| ",
    "(|_| ",
    "Option<",
    "Result<",
    "Ok(",
    "Err(",
    "Some(",
    "None",
    "struct ",
    "pub ",
    "enum ",
    "&[&str",
    ".collect::<",
    "let ",
    "let mut ",
];

fn probably_rust(s: &str) -> bool {
    RUST_SYNTAX.iter().any(|expr| s.contains(expr))
}

// TODO make a tokenizer + a const list tokens for every language
// tokenize input: if all input tokens are contained within that language's tokens list
// then mime type must be that token

// pub enum Language {
//     Ar(Dialect),
//     En(Dialect),
//     Fr(Dialect),
//     De(Dialect),
//     Jp(Dialect),
//     Cn(Dialect),
//     Ru(Dialect),
// }
//
// pub enum Dialect {
//     Us,
//     Ca,
//     Sa,
//     Ja,
// }
