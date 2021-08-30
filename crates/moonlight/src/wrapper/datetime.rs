use crate::*;

impl<Tz: TimeZone> From<Wrapper<Self>> for DateTime<Tz> {
    fn from(wrapper: Wrapper<Self>) -> Self {
        wrapper.inner
    }
}

#[cfg(feature = "serde-lite")]
impl<Tz: TimeZone> Serialize for Wrapper<DateTime<Tz>> where Tz::Offset: std::fmt::Display {
    fn serialize(&self) -> Result<Intermediate, serde_lite::Error> {
        self.inner.to_rfc3339().serialize()
    }
}

#[cfg(feature = "serde-lite")]
impl Deserialize for Wrapper<DateTime<Local>> {
    fn deserialize(intermediate: &Intermediate) -> Result<Self, serde_lite::Error> {
        let date_time = intermediate
            .as_str()
            .ok_or_else(|| {
                serde_lite::Error::invalid_value("DateTime can be deserialized only from String")
            })?;
        chrono::DateTime::parse_from_rfc3339(date_time)
            .map_err(|error| serde_lite::Error::invalid_value(error))
            .map(|date_time| Self::new(date_time.into()))
    }
}
