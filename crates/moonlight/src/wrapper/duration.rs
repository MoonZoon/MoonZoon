use crate::*;

impl From<Wrapper<Self>> for Duration {
    fn from(wrapper: Wrapper<Self>) -> Self {
        wrapper.inner
    }
}

impl Serialize for Wrapper<Duration> {
    fn serialize(&self) -> Result<Intermediate, serde_lite::Error> {
        self.inner.num_nanoseconds().serialize()
    }
}

impl Deserialize for Wrapper<Duration> {
    fn deserialize(intermediate: &Intermediate) -> Result<Self, serde_lite::Error> {
        intermediate
            .as_i64()
            .ok_or_else(|| {
                serde_lite::Error::invalid_value("Duration can be deserialized only from i64")
            })?
            .map(|nanoseconds| Self::new(Duration::nanoseconds(nanoseconds)))
    }
}
