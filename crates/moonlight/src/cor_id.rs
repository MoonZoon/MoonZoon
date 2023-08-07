use crate::*;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
