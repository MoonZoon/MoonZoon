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

#[cfg(feature = "serde-lite")]
impl Serialize for CorId {
    fn serialize(&self) -> Result<Intermediate, serde_lite::Error> {
        Ok(Intermediate::String(self.to_string()))
    }
}

#[cfg(feature = "serde-lite")]
impl Deserialize for CorId {
    fn deserialize(intermediate: &Intermediate) -> Result<Self, serde_lite::Error> {
        intermediate
            .as_str()
            .ok_or_else(|| {
                serde_lite::Error::invalid_value("CorId can be deserialized only from String")
            })?
            .parse()
            .map_err(|error| serde_lite::Error::invalid_value(error))
    }
}
