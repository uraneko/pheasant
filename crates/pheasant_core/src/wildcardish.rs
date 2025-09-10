#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum WildCardish<T> {
    /// *
    #[default]
    Glob,
    // ?
    // Quest,
    // Pattern(String),
    Value(T),
}

impl<T: PartialEq> WildCardish<T> {
    pub fn is_glob(&self) -> bool {
        self == &Self::Glob
    }
}

impl<T> WildCardish<T> {
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Self::Glob => None,
            Self::Value(t) => Some(t),
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        let Self::Value(t) = self else { return None };

        Some(t)
    }
}
