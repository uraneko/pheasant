use super::Token;
use crate::{SUB_DELIMS, SpellChecker, SpellingError as Error, UNRESERVED};
use hashbrown::{HashMap, HashSet};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Query {
    params: HashMap<String, String>,
    attrs: HashSet<String>,
}

impl SpellChecker for Query {
    const ALLOWED: &'static [char] = &[':', '@', '/', '?'];
    type Input<'a> = &'a [Token];

    fn spell_check(group: &[Token]) -> Result<(), Error> {
        if group.iter().any(|t| t.is_qmark() || t.is_pound()) {
            return Err(Error::InvalidTokenForComponent);
        }

        if group
            .iter()
            .filter_map(|t| match t {
                Token::Seq(s) => Some(s),
                _ => None,
            })
            .any(|seq| {
                seq.chars().any(|c| {
                    !c.is_alphanumeric()
                        && !Self::ALLOWED.contains(&c)
                        && !UNRESERVED.contains(&c)
                        && !SUB_DELIMS.contains(&c)
                })
            })
        {
            return Err(Error::InvalidCharsForComponent);
        }

        Ok(())
    }
}

impl Query {
    pub fn from_iter<I: IntoIterator<Item = Token>>(i: I) -> Self {
        let mut query = Query::default();
        let mut iter = i.into_iter().peekable();
        let mut key = String::new();
        let mut val = String::new();

        while iter.peek().is_some() {
            query.collect_key(&mut iter, &mut key);
            query.collect_val(&mut iter, &mut val, &mut key);
        }

        match [key.is_empty(), val.is_empty()] {
            [false, false] => {
                query.params.insert(key, val);
            }
            [false, true] => {
                query.attrs.insert(key);
            }
            _ => (),
        };

        query
    }
}

pub fn fragment_from_iter<I: IntoIterator<Item = Token>>(i: I) -> String {
    i.into_iter()
        .fold("".to_string(), |acc, t| acc + t.as_str())
}

impl Query {
    fn collect_key(&mut self, tokens: &mut impl Iterator<Item = Token>, key: &mut String) {
        while let Some(token) = tokens.next() {
            if token.is_eql() {
                return;
            } else if token.is_amper() {
                self.attrs.insert(key.drain(..).collect());

                return;
            }

            key.push_str(token.as_str());
        }
    }

    fn collect_val(
        &mut self,
        tokens: &mut impl Iterator<Item = Token>,
        val: &mut String,
        key: &mut String,
    ) {
        if key.is_empty() {
            return;
        }

        while let Some(token) = tokens.next() {
            if token.is_amper() {
                self.params
                    .insert(key.drain(..).collect(), val.drain(..).collect());
                return;
            }

            val.push_str(token.as_str());
        }
    }
}

impl Query {
    pub fn insert_param(&mut self, k: impl Into<String>, v: impl Into<String>) {
        self.params.insert(k.into(), v.into());
    }

    pub fn insert_attr<S>(&mut self, a: S)
    where
        S: Into<String>,
    {
        self.attrs.insert(a.into());
    }

    pub fn insert_iter_param<I>(&mut self, k: I, v: I)
    where
        I: IntoIterator<Item = char>,
    {
        self.params
            .insert(k.into_iter().collect(), v.into_iter().collect());
    }

    pub fn insert_iter_attr<I>(&mut self, a: I)
    where
        I: IntoIterator<Item = char>,
    {
        self.attrs.insert(a.into_iter().collect());
    }

    pub fn params(&self) -> &HashMap<String, String> {
        &self.params
    }

    pub fn attrs(&self) -> &HashSet<String> {
        &self.attrs
    }

    // borrows a param from self.params
    pub fn param(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|val| val.as_str())
    }

    /// removes a param from self.params
    pub fn take_param(&mut self, key: &str) -> Option<String> {
        self.params.remove(key)
    }

    pub fn contains_param(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }

    pub fn contains_attr(&self, attr: &str) -> bool {
        self.attrs.contains(attr)
    }
}

impl Query {
    pub fn new(params: impl Iterator<Item = String>) -> Self {
        Self::default()
    }

    pub fn from_colls(map: HashMap<&str, &str>, set: HashSet<&str>) -> Self {
        Query {
            params: map.into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
            attrs: set.into_iter().map(|a| a.into()).collect(),
        }
    }

    // returns the str repr of this query
    pub fn serialized(&self) -> String {
        let mut seq = self
            .params
            .iter()
            .fold("".to_owned(), |acc, (k, v)| acc + k + "=" + v + "&");
        seq = self.attrs.iter().fold(seq, |acc, a| acc + a + "&");
        seq.insert(0, '?');
        seq.pop();

        seq
    }
}

impl std::str::FromStr for Query {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut query = Query::default();
        str_to_pairs(&mut query, s);

        Ok(query)
    }
}

// parses the query params into key -> value pairs
fn str_to_pairs(query: &mut Query, s: &str) {
    s.split('&')
        // BUG this crashes the server when uri query is badly formatted
        // TODO scan query after getting request and return ClientError::BadRequest if query is faulty
        .map(|e| str_to_pair(e))
        .for_each(|[k, v]| {
            if v.is_empty() {
                query.insert_attr(k);
            } else {
                query.insert_param(k, v);
            }
        });
}

// NOTE this handles the pain points of parse_query
// the check for `=` garentees the operation's success
fn str_to_pair(p: &str) -> [&str; 2] {
    if p.contains('=') {
        p.splitn(2, '=').collect::<Vec<&str>>().try_into().unwrap()
    } else {
        // TODO possibly make a HashSet of bool params alongside the HashMap of k -> v pairs
        [p, ""]
    }
}
