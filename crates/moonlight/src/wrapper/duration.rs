use crate::*;
use std::convert::TryInto;

impl From<Wrapper<Self>> for Duration {
    fn from(wrapper: Wrapper<Self>) -> Self {
        wrapper.inner
    }
}

impl Default for Wrapper<Duration> {
    fn default() -> Self {
        Wrapper::new(Duration::zero())
    }
}

#[cfg(feature = "serde")]
impl Serialize for Wrapper<Duration> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if let Some(nanoseconds) = self.inner.num_nanoseconds() {
            serializer.serialize_i64(nanoseconds)
        } else {
            Err(ser::Error::custom("too much nanoseconds for i64"))
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Wrapper<Duration> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct I64NanosecondsVisitor;

        impl<'de> de::Visitor<'de> for I64NanosecondsVisitor {
            type Value =  i64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("nanoseconds representable as i64")
            }

            fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
                Ok(i64::from(value))
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
                value
                    .try_into()
                    .map_err(de::Error::custom)
            }

            fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
                Ok(value as Self::Value)
            }
        }

        let nanoseconds= deserializer.deserialize_i64(I64NanosecondsVisitor)?;
        Ok(Wrapper::new(Duration::nanoseconds(nanoseconds)))
    }
}

#[cfg(feature = "serde-lite")]
impl Serialize for Wrapper<Duration> {
    fn serialize(&self) -> Result<Intermediate, serde_lite::Error> {
        self.inner.num_nanoseconds().serialize()
    }
}

#[cfg(feature = "serde-lite")]
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
