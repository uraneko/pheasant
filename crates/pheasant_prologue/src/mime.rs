use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mime(mime::Mime);

impl Mime {
    // safe unwrap as long as this function is used as intended,
    // which is from the http methods macros
    pub fn macro_checked(s: &str) -> Self {
        s.parse::<Mime>().unwrap()
    }
}

impl std::ops::Deref for Mime {
    type Target = mime::Mime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Mime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<mime::Mime> for Mime {
    fn from(m: mime::Mime) -> Self {
        Self(m)
    }
}

impl From<Mime> for mime::Mime {
    fn from(m: Mime) -> Self {
        m.0
    }
}

impl std::str::FromStr for Mime {
    type Err = mime::FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<mime::Mime>()?))
    }
}

impl fmt::Display for Mime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.essence_str())
    }
}

impl Default for Mime {
    fn default() -> Self {
        Self(mime::APPLICATION_OCTET_STREAM)
    }
}

#[derive(Debug)]
enum MimeError {
    MimeError,
}

impl core::error::Error for MimeError {}

impl std::fmt::Display for MimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// impl ToTokens for Mime {
//     fn to_tokens(&self, tokens: &mut TS2) {
//         tokens.append(<&Mime as Into<TokenTree>>::into(self))
//     }
// }
//
// impl From<&Mime> for TokenTree {
//     fn from(m: &Mime) -> Self {
//         let mut ts = TS2::new();
//         let ident = Ident::new("Mime", Span::call_site());
//         ts.append(ident);
//
//         let lit = Group::new(
//             Delimiter::Parenthesis,
//             TokenTree::Literal(Literal::string(m.essence_str())).into(),
//         );
//         ts.append(lit);
//
//         let group = Group::new(Delimiter::None, ts);
//         TokenTree::from(group)
//     }
// }
