use crate::*;
use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EntityId(Ulid);

impl EntityId {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self(Ulid::generate())
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for EntityId {
    type Err = DecodingError;

    fn from_str(entity_id: &str) -> Result<Self, Self::Err> {
        Ok(EntityId(entity_id.parse()?))
    }
}
