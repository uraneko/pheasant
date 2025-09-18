// pub mod authentication;
// pub mod caching;
// pub mod conditionals;
// pub mod connection_management;
// pub mod content_negotiation;
// pub mod controls;
pub mod cookies;
pub mod cors;
// pub mod deprecated;
// pub mod downloads;
// pub mod experimental;
// pub mod fetch_metadata_request_headers;
// pub mod integrity_digests;
// pub mod integrity_policy;
pub mod message_body_information;
// pub mod non_standard;
pub mod other;
// pub mod preferences;
// pub mod proxies;
// pub mod range_requests;
// pub mod redirects;
pub mod request_context;
// pub mod response_context;
// pub mod security;
// pub mod server_sent_events;
// pub mod transfer_coding;
// pub mod websockets;

pub use cookies::Cookie;
pub use cors::{RequestCors, ResourceCors, ResponseCors};
pub use message_body_information::{
    ContentEncoding, ContentEncodingBits, ContentLength, ContentType, EncodeBody, Encoding,
    SetContentLength,
};
pub use other::{Date, SetDate};
pub use request_context::Host;

struct Headers {
    entries: [Entry; 64],
    size: usize,
}

// hash can be generated from key
// Key -> Hash
// index can be generated from hash
// Hash -> Index
//
// Hash should be generated from key + unique + len/index
// Cookie -> 53906 -> 539061 ->
// gets hashed -> add unique -> rehash by index if not unique
//
// Condition if the header is unique then hashing Header+Unique should give us the index
// if header is repeating then

// entries -> [ContentTypeValue, ContentLengthValue, CookieValue, OriginValue]
// indexes -> [ContentTypeHash,  ContentLengthHash,  CookieHash,  OriginHash]
// headers.insert(cookie)
// => entries -> [ContentTypeValue, ContentLengthValue, CookieValue, OriginValue, CookieValue]
//    indexes -> [ContentTypeHash , ContentLengthHash , CookieHash ,               OriginHash]
// need to know where each header index starts

// struct Entry {
//     value: Vec<u8>
// }
//
// impl Entry {
//     // e.g., Content-Type -> 34232
//     const HeaderHash: u64;
//     const Unique: bool;
//     const Hash HeaderHash + Unique
// }
