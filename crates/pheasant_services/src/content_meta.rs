use mime::Mime;

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

    pub fn from_params(len: usize, ty: Mime) -> Self {
        Self { len, ty }
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

    pub fn force_mime(mut self, ty: Mime) -> Self {
        self.ty = ty;

        self
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

pub fn guess_mime(data: &[u8]) -> Mime {
    let Ok(s) = str::from_utf8(data) else {
        return mime::APPLICATION_OCTET_STREAM;
    };

    match s.trim() {
        s if s.starts_with('{') && s.ends_with('}') => mime::APPLICATION_JSON,
        s if s.starts_with("<!DOCTYPE html>") => mime::TEXT_HTML,
        _ => mime::TEXT_PLAIN,
    }
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
