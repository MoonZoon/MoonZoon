pub use serde_lite::{self, Serialize, Deserialize, Intermediate};
pub use serde_json;
use std::{fmt, str::FromStr};
pub use rusty_ulid::{self, Ulid, DecodingError};

// ------ SessionId ------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SessionId(Ulid);

impl SessionId {
    pub fn new() -> Self {
        SessionId(Ulid::generate())
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SessionId {
    type Err = DecodingError;

    fn from_str(session_id: &str) -> Result<Self, Self::Err> {
        Ok(SessionId(session_id.parse()?))
    }
}

// ------ CorId ------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl Serialize for CorId {
    fn serialize(&self) -> Result<Intermediate, serde_lite::Error> {
        Ok(Intermediate::String(self.to_string()))
    }
}

impl Deserialize for CorId {
    fn deserialize(itermediate: &Intermediate) -> Result<Self, serde_lite::Error> {
        itermediate
            .as_str()
            .ok_or_else(|| {
                serde_lite::Error::invalid_value("CorId can be deserialized only from String")
            })?
            .parse()
            .map_err(|error| {
                serde_lite::Error::invalid_value(error)
            })
    }
}

// ------ AuthToken ------

#[derive(Debug, Clone)]
pub struct AuthToken(String);

impl AuthToken {
    pub fn new(token: impl ToString) -> Self {
        AuthToken(token.to_string())
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ------ DownMsgTransporter ------

#[derive(Serialize)]
pub struct DownMsgTransporterForSer<'a, DMsg: Serialize> {
    pub down_msg: &'a DMsg,
    pub cor_id: CorId,
}

#[derive(Deserialize)]
pub struct DownMsgTransporterForDe<DMsg: Deserialize> {
    pub down_msg: DMsg,
    pub cor_id: CorId,
}
