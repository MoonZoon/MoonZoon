use std::ops::Deref;
use moonlight::{Ulid, serde_lite::{self, Serialize, Deserialize}};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct EntityId(Ulid);

impl EntityId {
    pub fn new() -> Self {
        Self(Ulid::generate())
    }
}

impl Deref for EntityId {
    type Target = Ulid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for EntityId {
    fn serialize(&self) -> Result<serde_lite::Intermediate, serde_lite::Error> {
        self.0.to_string().serialize()
    }
}

impl Deserialize for EntityId {
    fn deserialize(intermediate: &serde_lite::Intermediate) -> Result<Self, serde_lite::Error> {
        intermediate
            .as_str()
            .ok_or_else(|| {
                serde_lite::Error::invalid_value("EntityId can be deserialized only from String")
            })?
            .parse()
            .map_err(|error| serde_lite::Error::invalid_value(error))
            .map(Self)
    }
}
