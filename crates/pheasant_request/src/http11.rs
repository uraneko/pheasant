extern crate std;
use std::io::{BufReader, Read};

use super::Request;
use pheasant_core::err_stt;

pub mod lex;
pub mod read;
pub mod scrutinize;

pub use lex::{Token, lex};

pub fn parse(stream: &mut impl Read, buffer: &mut Vec<u8>) {
    let reader = BufReader::new(stream);
    let tokens = lex(reader, &mut buffer);
    if tokens.is_empty() {
        return err_stt!(?BadRequest);
    }

    Request::parse(tokens)
}

// request contains Request & Builder & http11
// http11 contains parse & lex & Token
