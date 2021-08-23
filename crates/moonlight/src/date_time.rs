use crate::*;
use std::{ops::Deref, fmt};

pub struct DateTime<Tz: TimeZone> {
    inner: chrono::DateTime<Tz>
}

impl<Tz: TimeZone> Deref for DateTime<Tz> {
    type Target = chrono::DateTime<Tz>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<Tz: TimeZone> From<chrono::DateTime<Tz>> for DateTime<Tz> {
    fn from(duration: chrono::DateTime<Tz>) -> Self {
        Self { inner: duration }
    }
}

impl<Tz: TimeZone> From<DateTime<Tz>> for chrono::DateTime<Tz> {
    fn from(duration: DateTime<Tz>) -> Self {
        duration.inner
    }
}

impl<Tz: TimeZone> Serialize for DateTime<Tz> where Tz::Offset: fmt::Display {
    fn serialize(&self) -> Result<Intermediate, serde_lite::Error> {
        self.inner.to_rfc3339().serialize()
    }
}

impl Deserialize for DateTime<Local> {
    fn deserialize(intermediate: &Intermediate) -> Result<Self, serde_lite::Error> {
        let date_time = intermediate
            .as_str()
            .ok_or_else(|| {
                serde_lite::Error::invalid_value("DateTime can be deserialized only from String")
            })?;
        chrono::DateTime::parse_from_rfc3339(date_time)
            .map_err(|error| serde_lite::Error::invalid_value(error))
            .map(|date_time| Self { inner: date_time.into() })
    }
}
