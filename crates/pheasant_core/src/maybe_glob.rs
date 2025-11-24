extern crate alloc;
use crate::{ErrorStatus, err_stt};
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::string::ToString;
use pheasant_uri::Origin;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum MaybeGlob<T> {
    /// *
    #[default]
    Glob,
    // ?
    // Quest,
    // Pattern(String),
    Value(T),
}

impl<T: PartialEq> MaybeGlob<T> {
    pub fn is_glob(&self) -> bool {
        self == &Self::Glob
    }
}

impl<T> MaybeGlob<T> {
    pub fn as_ref(&self) -> MaybeGlob<&T> {
        match self {
            Self::Glob => MaybeGlob::Glob,
            Self::Value(v) => MaybeGlob::Value(v),
        }
    }

    pub fn as_mut(&mut self) -> MaybeGlob<&mut T> {
        match self {
            Self::Glob => MaybeGlob::Glob,
            Self::Value(v) => MaybeGlob::Value(v),
        }
    }

    pub fn maybe_ref(&self) -> Option<&T> {
        match self {
            Self::Glob => None,
            Self::Value(t) => Some(t),
        }
    }

    pub fn maybe_mut(&mut self) -> Option<&mut T> {
        let Self::Value(t) = self else { return None };

        Some(t)
    }
}

impl<'a, T> Copy for MaybeGlob<&'a T> {}

impl TryFrom<String> for MaybeGlob<Origin> {
    type Error = ErrorStatus;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse::<Self>()
    }
}

impl core::str::FromStr for MaybeGlob<Origin> {
    type Err = ErrorStatus;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "*" {
            return Ok(MaybeGlob::Glob);
        }

        s.parse::<Origin>()
            .map(|o| MaybeGlob::Value(o))
            .map_err(|_| err_stt!(BadRequest))
    }
}

impl From<MaybeGlob<&Origin>> for String {
    fn from(h: MaybeGlob<&Origin>) -> String {
        match h {
            MaybeGlob::Glob => "*".to_owned(),
            MaybeGlob::Value(o) => o.to_string(),
        }
    }
}

impl<T: core::fmt::Display + PartialEq> core::fmt::Display for MaybeGlob<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            if self.is_glob() {
                "*".into()
            } else {
                self.maybe_ref()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "failed to print T".into())
            }
        )
    }
}
