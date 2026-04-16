use hashbrown::HashSet;

#[derive(Debug, Default, Clone)]
pub struct ContentSecurityPolicy<'a> {
    policies: HashSet<ContentSecurity<'a>>,
}

impl<'a> ContentSecurityPolicy<'a> {
    pub fn policy(mut self, policy: ContentSecurity<'a>) -> Self {
        self.policies.insert(policy);

        self
    }

    pub fn policies(mut self, policies: impl IntoIterator<Item = ContentSecurity<'a>>) -> Self {
        self.policies.extend(policies);

        self
    }

    pub fn write_header(&self, buf: &mut Vec<u8>) {
        buf.extend(b"content-security-policy");
        for policy in self.policies.iter() {
            buf.extend(policy.stream_bytes());
            buf.push(b';');
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FetchDirective {
    #[default]
    Default = 0,
    Image = 1,
    Script = 2,
    Style = 4,
}

impl FetchDirective {
    // pub fn as_u8(&self) -> u8 {
    //     match self {
    //         Self::Default => 0,
    //         Self::Image => 1,
    //         Self::Script => 2,
    //         Self::Style => 4,
    //     }
    // }

    pub fn as_bytes(&self) -> &'static [u8] {
        match self {
            Self::Default => b"default-src",
            Self::Image => b"img-src",
            Self::Script => b"script-src",
            Self::Style => b"style-src",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentSecurity<'a> {
    directive: FetchDirective,
    origins: HashSet<&'a str>,
    same_origin: bool,
}

impl<'a> core::hash::Hash for ContentSecurity<'a> {
    fn hash<H>(&self, state: &mut H)
    where
        H: core::hash::Hasher,
    {
        self.directive.hash(state);
    }
}

impl<'a> Default for ContentSecurity<'a> {
    fn default() -> Self {
        Self {
            directive: FetchDirective::default(),
            origins: HashSet::new(),
            same_origin: true,
        }
    }
}

impl<'a> ContentSecurity<'a> {
    /// makes a new instance with the provided fetch directive
    pub fn new(directive: FetchDirective) -> Self {
        Self {
            directive,
            origins: HashSet::new(),
            same_origin: true,
        }
    }

    /// disables the self source expression, which is enabled by default
    pub fn no_self(mut self) -> Self {
        self.same_origin = false;

        self
    }

    /// adds a new origin to the allowed source expressions
    pub fn origin(mut self, origin: &'a str) -> Self {
        self.origins.insert(origin);

        self
    }

    /// adds many origins to the allowed source expressions
    pub fn origins(mut self, origins: &[&'a str]) -> Self {
        self.origins.extend(origins);

        self
    }

    pub fn stream_bytes(&self) -> impl IntoIterator<Item = u8> {
        self.directive
            .as_bytes()
            .into_iter()
            .chain(
                self.same_origin
                    .then(|| b" 'self'".as_slice())
                    .unwrap_or_default()
                    .into_iter(),
            )
            .copied()
            .chain(stream_origins(&self.origins).into_iter())
    }
}

fn stream_origins<'a>(origins: &HashSet<&'a str>) -> Vec<u8> {
    let mut v = Vec::new();
    let mut last = origins.len() - 1;
    let mut iter = origins.into_iter();
    while let Some(o) = iter.next()
        && last > 0
    {
        if last == 0 {
            break;
        }
        v.extend(o.as_bytes());
        v.push(32);
        last -= 1;
    }
    let Some(o) = iter.next() else {
        unreachable!("there is still 1 origin remaining")
    };
    v.extend(o.as_bytes());

    v
}
