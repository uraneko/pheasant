use super::Request;
use pheasant_core::WildCardish;
use pheasant_headers::{ContentLength, ContentType, Cookie, Date, Host, RequestCors};
use pheasant_uri::Origin;

fn fn_to_header(f: &str) -> String {
    let mut h = f.to_owned();
    if !f.contains("_") {
        return h;
    }

    let snakes = f
        .chars()
        .enumerate()
        .filter(|(_, c)| c == '_')
        .for_each(|(i, c)| {
            // ends with _
            if i > h.len() - 2 {
                return;
            }

            h.remove(i);
            let capi = h.remove(i).to_uppercase();
            h.insert(i, capi);
        });

    h
}

macro_rules! fn_to_header {
    ($s: ident) => {
        &fn_to_header(stringify!($s))
    };
}

macro_rules! header {
    ($n: ident ( $t: ty )) => {
        pub fn $n(&mut self) -> Option<$t> {
            self.headers
                .remove(stringify!($n))
                .map(|s| <$t>::from_header(s))
        }
    };
}

macro_rules! header_group {
    ($n: ident [ $t: ty ]) => {
        pub fn $n(&self) -> Option<$t> {
            <$t>::from_headers(&mut self.headers)
        }
    };
}

macro_rules! headers {
    ($($n: ident ( $t : ty )),+) => {
        $(
            header!($n ($t ));
        )*
    };
    ($($n: ident [ $t: ty ]),+) => {
        $(
            header_group!($n [ $t ]);
        )*
    };
}

impl Request {
    headers!(
        // should be DateTime<Utc>
        date(Date),
        // Mime
        content_type(ContentType),
        // usize
        content_length(ContentLength),
        // should be Origin
        host(Host),
        origin(WildCardish<Origin>)
    );

    headers!(cors[RequestCors], cookies[ HashSet<Cookie> ] );
}
