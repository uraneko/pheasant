use mime::{FromStrError, Mime};

pub struct MessageBodyInfo {
    ty: Mime,
    len: usize,
    // language: Option<Language>,
}

impl MessageBodyInfo {
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

    pub fn dump_headers(self, headers: &mut Vec<u8>) {
        headers.extend(b"content-length: ");
        headers.extend(self.len.to_string().as_bytes());
        headers.push(10);
        headers.extend(b"content-type: ");
        headers.extend(self.ty.essence_str().as_bytes());
        headers.push(10);
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
        s if s.starts_with('{') && s.ends_with('}') => mime::APPLICATION_JSON,
        s if s.starts_with("<!DOCTYPE html>") => mime::TEXT_HTML,
        s if css_syntaxful(s) => mime::TEXT_CSS,
        s if s.starts_with("<?xml ") && s.ends_with("</svg>") => mime::IMAGE_SVG,
        s if js_syntaxful(s) => mime::TEXT_JAVASCRIPT,
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

fn css_syntaxful(s: &str) -> bool {
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

fn js_syntaxful(s: &str) -> bool {
    JS_SYNTAX.iter().any(|expr| s.contains(expr))
}

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
