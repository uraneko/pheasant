use crate::{Component, PercentEncoded, Sanitizer, SpellChecker};

#[derive(Debug)]
pub struct Host {
    labels: Vec<String>,
}

impl Host {
    const LABEL_MAX: usize = 63;

    pub fn new(labels: impl Iterator<Item = String>) -> Self {
        Self {
            labels: labels.collect(),
        }
    }

    fn label_too_long(label: &str) -> bool {
        label.len() > Self::LABEL_MAX
    }
}

impl Component for Host {
    const ALLOWED: &[u8] = &[
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
        b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd',
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
        b'8', b'9', b'-',
    ];

    const MAX_LEN: usize = 255;

    fn is_forbidden(ch: u8) -> bool {
        (ch <= b'9' && ch >= b'0') || (ch >= b'a' && ch <= b'z') || (ch >= b'A' && ch <= b'Z')
    }
}

impl SpellChecker for Host {
    // spell check for a single host label
    fn spell_check(s: &str) -> Result<(), ()> {
        if s.len() > Self::LABEL_MAX {
            return Err(());
        } else if s.starts_with('-') || s.ends_with('-') {
            return Err(());
        } else if s.chars().all(|ch| !Self::is_forbidden(ch as u8)) {
            Ok(())
        } else {
            Err(())
        }
    }
}
