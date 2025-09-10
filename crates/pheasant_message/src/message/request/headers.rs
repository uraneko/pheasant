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
    ($t: ty, $n: ident) => {
        pub fn $n(&self) -> Option<$t> {
            self.headers
                .remove(fn_to_header!($n))
                .map(|s| s.from_header())
        }
    };
}

macro_rules! header_group {
    ($t: ty, $n: ident ) => {
        pub fn $n(&self) -> Option<$t> {
            $t::from_headers(&mut self.headers)
        }
    };
}

macro_rules! headers {
    ($($t: ty { $n: ident }),+) => {
        $(
            header!($t, $n);
        )*
    };
    ($($t: ty [ $n: ident ]),+) => {
        $(
            header_group!($t, $n);
        )*
    };
}

impl Request {
    headers!(
        // DateTime<Utc>
        Date { date },
        // Mime
        ContentType { content_type },
        // usize
        ContentLength { content_length },
        // should be Origin
        Host { host },
        WildCardish<Origin> { origin },
    );

    headers!(RequestCors[cors]);
    headers!(HashSet<Cookie>[cookies]);
}
