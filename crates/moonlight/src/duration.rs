use crate::*;
use std::ops::Deref;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Duration {
    pub inner: chrono::Duration
}

impl Deref for Duration {
    type Target = chrono::Duration;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<chrono::Duration> for Duration {
    fn from(duration: chrono::Duration) -> Self {
        Self { inner: duration }
    }
}

impl From<Duration> for chrono::Duration {
    fn from(duration: Duration) -> Self {
        duration.inner
    }
}

impl Serialize for Duration {
    fn serialize(&self) -> Result<Intermediate, serde_lite::Error> {
        self.inner.num_nanoseconds().serialize()
    }
}

impl Deserialize for Duration {
    fn deserialize(intermediate: &Intermediate) -> Result<Self, serde_lite::Error> {
        intermediate
            .as_i64()
            .ok_or_else(|| {
                serde_lite::Error::invalid_value("Duration can be deserialized only from i64")
            })?
            .map(|nanoseconds| Self { inner: chrono::Duration::nanoseconds(nanoseconds) })
    }
}
