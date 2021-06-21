pub use serde_lite;
pub use serde_json_wasm;
use std::{fmt, str::FromStr};
pub use rusty_ulid::{self, Ulid, DecodingError};

// ------ CorId ------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CorId(Ulid);

impl CorId {
    pub fn new() -> Self {
        CorId(Ulid::generate())
    }
}

impl fmt::Display for CorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CorId {
    type Err = DecodingError;

    fn from_str(cor_id: &str) -> Result<Self, Self::Err> {
        Ok(CorId(cor_id.parse()?))
    }
}

// ------ AuthToken ------

#[derive(Debug, Clone)]
pub struct AuthToken(String);

impl AuthToken {
    pub fn new(token: impl ToString) -> Self {
        AuthToken(token.to_string())
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
