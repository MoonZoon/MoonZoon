use crate::*;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SessionId(Ulid);

impl SessionId {
    pub fn new() -> Self {
        SessionId(Ulid::generate())
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SessionId {
    type Err = DecodingError;

    fn from_str(session_id: &str) -> Result<Self, Self::Err> {
        Ok(SessionId(session_id.parse()?))
    }
}
