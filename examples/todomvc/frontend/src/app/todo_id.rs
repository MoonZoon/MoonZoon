use crate::*;
use std::ops::Deref;
use uuid::Uuid;

// ------ TodoId -------

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "serde")]
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
