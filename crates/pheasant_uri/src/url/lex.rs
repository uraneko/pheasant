macro_rules! token {
    ($s: expr) => {
        match $s {
            '/' => Token::Slash,
            ':' => Token::Colon,
            '?' => Token::QuestionMark,
            '#' => Token::Pound,
            '@' => Token::AddressSign,
            '=' => Token::Equality,
            '&' => Token::AmperSand,
            '.' => Token::Dot,
            _ => panic!("declmacro, unexpected char token value"),
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Token {
    Seq(String),
    Slash,
    Dot,
    Colon,
    QuestionMark,
    Pound,
    AddressSign,
    AmperSand,
    Equality,
}

macro_rules! token_is {
    ($name: ident, $var: ident ()) => {
        pub fn $name(&self) -> bool {
            let Self::$var(_) = self else { return false };

            true
        }
    };
    ($name: ident, $var: ident) => {
        pub fn $name(&self) -> bool {
            let Self::$var = self else { return false };

            true
        }
    };
}

impl Token {
    token_is!(is_seq, Seq());
    token_is!(is_dot, Dot);
    token_is!(is_qmark, QuestionMark);
    token_is!(is_pound, Pound);
    token_is!(is_eql, Equality);
    token_is!(is_colon, Colon);
}

impl Token {
    pub fn seq(s: &str) -> Self {
        Self::Seq(s.to_owned())
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Seq(s) => s,
            Self::QuestionMark => "?",
            Self::Pound => "#",
            Self::Colon => ":",
            Self::Slash => "/",
            Self::AddressSign => "@",
            Self::Equality => "=",
            Self::AmperSand => "&",
            Self::Dot => ".",
        }
    }

    fn to_char(&self) -> Option<char> {
        match self {
            Self::Seq(_) => None,
            token => Some(match token {
                Self::Seq(_) => unreachable!("matched out 2 lines ago"),
                Self::QuestionMark => '?',
                Self::Pound => '#',
                Self::Colon => ':',
                Self::Slash => '/',
                Self::AddressSign => '@',
                Self::Equality => '=',
                Self::AmperSand => '&',
                Self::Dot => '.',
            }),
        }
    }

    pub fn seq_str(self) -> Option<String> {
        let Self::Seq(s) = self else {
            return None;
        };

        Some(s)
    }
}

#[derive(Debug)]
pub enum Error {
    ExpectedTokensFoundNone,
}

const SEPS: [char; 8] = ['@', '/', ':', '?', '#', '=', '&', '.'];

fn find_all(mut s: &str, ch: char) -> Vec<(usize, char)> {
    let mut v = vec![];
    let mut last = 0;

    while let Some(idx) = s.find(ch) {
        v.push((idx + last, ch));
        last += idx + 1;
        s = &s[idx + 1..];
    }

    v
}

// TODO return Result + error accordingly following the standard
pub fn lex(mut s: &str) -> Result<Vec<Token>, Error> {
    let mut breakpoints = SEPS
        .into_iter()
        .map(|sep| find_all(s, sep))
        .flatten()
        .collect::<Vec<(usize, char)>>();
    breakpoints.sort_by(|a, b| a.0.cmp(&b.0));

    let mut last = 0;
    // NOTE not sure why i return Option::Some when I will never return None
    let mut v = breakpoints
        .into_iter()
        .map(|(idx, ch)| {
            let toks = if idx > last {
                Some(vec![Token::seq(&s[..idx - last]), token!(ch)])
            } else {
                Some(vec![token!(ch)])
            };

            s = &s[idx + 1 - last..];
            last = idx + 1;

            toks
        })
        // .map(|toks| toks.ok_or_else(|| Error::ExpectedTokensFoundNone))
        .flatten()
        .flatten()
        .collect::<Vec<Token>>();

    if !s.is_empty() {
        v.push(Token::seq(s));
    }

    Ok(v)
}
