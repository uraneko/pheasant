use chrono::{DateTime, TimeDelta, Utc};
use hashbrown::HashMap;
use pheasant_http::Header;
use pheasant_uri::{Host, Path};

pub struct ReadCookies {
    cookies: HashMap<Vec<u8>, Vec<u8>>,
}

impl ReadCookies {
    pub fn from_headers(headers: &[Header]) -> Result<Self, Error> {
        let mut cookies = HashMap::new();
        let mut headers = headers
            .into_iter()
            .filter_map(|h| (h.field_ref() == b"cookie").then(|| h.value_ref()));

        while let Some(cookie) = headers.next() {
            parse_header(cookie, &mut cookies)?;
        }

        Ok(Self { cookies })
    }

    pub fn len(&self) -> usize {
        self.cookies.len()
    }

    /// checks if a value exists inside the nameless field: vec![]
    pub fn contains_attr(&self, val: &[u8]) -> bool {
        let Some(value) = self.cookies.get(&vec![]) else {
            return false;
        };

        slice_contains(value, val)
    }
}

/// checks if a slice contains a subslice or not
pub fn slice_contains(slice: &[u8], subslice: &[u8]) -> bool {
    let len = subslice.len();
    if slice.len() < len {
        return false;
    }

    let mut bytes = slice.into_iter();
    let mut idx = 0;
    while let Some(b) = bytes.next() {
        if *b == subslice[idx] {
            if idx == len - 1 {
                return true;
            }

            idx += 1;
        } else {
            idx = 0;
        }
    }

    false
}

impl Into<WriteCookies> for ReadCookies {
    fn into(self) -> WriteCookies {
        WriteCookies {
            cookies: self
                .cookies
                .into_iter()
                .map(|(f, v)| (f, (v, CookieParams::new())))
                .collect(),
        }
    }
}

impl Into<ReadCookies> for WriteCookies {
    fn into(self) -> ReadCookies {
        ReadCookies {
            cookies: self.cookies.into_iter().map(|(f, (v, _))| (f, v)).collect(),
        }
    }
}

pub struct WriteCookies {
    cookies: HashMap<Vec<u8>, (Vec<u8>, CookieParams)>,
}

impl WriteCookies {
    /// creates a new cookie with field and value
    /// then returns a mutable borrow to the new cookie's params
    pub fn cookie(&mut self, field: &[u8], value: &[u8]) -> &mut CookieParams {
        self.cookies
            .insert(field.to_vec(), (value.to_vec(), CookieParams::new()));

        // safe unwrap
        self.params_mut(field).unwrap()
    }

    /// renames the cookie with the 'field' name to 'new'
    pub fn rename(&mut self, field: &[u8], new: &[u8]) {
        let Some(data) = self.cookies.remove(field) else {
            return;
        };

        self.cookies.insert(new.to_vec(), data);
    }

    /// updates the 'field' cookie's value with the 'value' slice
    pub fn update(&mut self, field: &[u8], value: &[u8]) {
        let Some((v, _)) = self.cookies.get_mut(field) else {
            return;
        };

        *v = value.to_vec();
    }

    pub fn value_mut(&mut self, field: &[u8]) -> Option<&mut Vec<u8>> {
        self.cookies.get_mut(field).map(|(v, _)| v)
    }
    // returns a mutable reference to a cookie's params
    pub fn params_mut(&mut self, field: &[u8]) -> Option<&mut CookieParams> {
        self.cookies.get_mut(field).map(|(_, p)| p)
    }

    pub fn len(&self) -> usize {
        self.cookies.len()
    }
}

pub struct CookieParams {
    prefix: Option<Prefix>,
    domain: Option<Host>,
    expires: Option<DateTime<Utc>>,
    http_only: bool,
    // has precedence over the expires property
    max_age: i64,
    // requires the secure attrbute to be set
    partitioned: bool,
    path: Option<Path>,
    samesite: SameSite,
    secure: bool,
}

// builder methods
impl CookieParams {
    pub fn new() -> Self {
        Self {
            prefix: None,
            domain: None,
            expires: None,
            http_only: false,
            max_age: 0,
            partitioned: false,
            path: None,
            samesite: SameSite::None,
            secure: false,
        }
    }

    pub fn prefix(&mut self, prefix: impl Into<Prefix>) -> &mut Self {
        self.prefix = Some(prefix.into());

        self
    }

    /// e.g., domain property
    /// Domain=some-company.co.uk
    pub fn domain(&mut self, domain: impl Into<Host>) -> &mut Self {
        self.domain = Some(domain.into());

        self
    }

    // e.g., expires property
    // Expires=Wed, 21 Oct 2015 07:28:00 GMT
    pub fn expires(&mut self, expires: DateTime<Utc>) -> &mut Self {
        self.expires = Some(expires);

        self
    }

    pub fn http_only(&mut self, http_only: bool) -> &mut Self {
        self.http_only = http_only;

        self
    }

    // e.g., max-age property
    // Max-Age=2592000
    pub fn max_age(&mut self, max_age: TimeDelta) -> &mut Self {
        self.max_age = max_age.num_seconds();

        self
    }

    pub fn partitioned(&mut self, partitioned: bool) -> &mut Self {
        self.partitioned = partitioned;
        if partitioned {
            self.secure = true;
        }

        self
    }

    pub fn path(&mut self, path: impl Into<Path>) -> &mut Self {
        self.path = Some(path.into());

        self
    }

    // e.g., samesite property
    // SameSite=None
    pub fn samesite(&mut self, ss: impl Into<SameSite>) -> &mut Self {
        self.samesite = ss.into();

        self
    }

    pub fn secure(&mut self, secure: bool) -> &mut Self {
        self.secure = secure;

        self
    }
}

pub enum Prefix {
    Secure,
    Host,
    Http,
    HostHttp,
}

pub enum SameSite {
    Strict,
    Lax,
    None,
}

pub fn parse_header(slice: &[u8], map: &mut HashMap<Vec<u8>, Vec<u8>>) -> Result<(), Error> {
    if slice.contains(&b';') {
        parse_cookies(slice, map)?;
    } else {
        parse_cookie(slice, map)?;
    }

    Ok(())
}

pub fn parse_cookies(slice: &[u8], map: &mut HashMap<Vec<u8>, Vec<u8>>) -> Result<(), Error> {
    let slice = slice.trim_ascii();
    let mut cookies = slice.split(|b| *b == b';');
    while let Some(cookie) = cookies.next() {
        parse_cookie(cookie, map)?
    }

    Ok(())
}

pub enum Error {
    EqualsNotFound,
}

pub fn parse_cookie(slice: &[u8], map: &mut HashMap<Vec<u8>, Vec<u8>>) -> Result<(), Error> {
    let slice = slice.trim_ascii();
    let Some(eql) = slice.iter().position(|b| *b == b'=') else {
        return Err(Error::EqualsNotFound);
    };

    let field = &slice[..eql];
    let value = &slice[eql + 1..];
    // all values that have no field name
    // are put under the same map key entry -> vec![]
    // and separated by b';'
    if field.is_empty()
        && let Some(val) = map.get_mut(field)
    {
        val.push(b';');
        val.extend(value);
    } else {
        map.insert(field.to_vec(), value.to_vec());
    }

    Ok(())
}
