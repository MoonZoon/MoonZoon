use crate::*;
use std::ops::Deref;
use uuid::Uuid;

// ------ TodoId -------

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct TodoId(Uuid);

impl TodoId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Deref for TodoId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for TodoId {
    fn serialize(&self) -> Result<serde_lite::Intermediate, serde_lite::Error> {
        self.0.to_string().serialize()
    }
}

impl Deserialize for TodoId {
    fn deserialize(intermediate: &serde_lite::Intermediate) -> Result<Self, serde_lite::Error> {
        intermediate.as_str()
            .ok_or_else(|| {
                serde_lite::Error::invalid_value("TodoId can be deserialized only from String")
            })?
            .parse()
            .map_err(|error| serde_lite::Error::invalid_value(error))
            .map(Self)
    }
}
