use serde_lite::{Deserialize, Serialize};
use rusty_ulid as ulid;

// ------ Message ------

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub username: String,
    pub text: String,
}

// ------ UpMsg ------

#[derive(Serialize, Deserialize)]
pub enum UpMsg {
    SendMessage(Message),
}

// ------ DownMsg ------

#[derive(Serialize, Deserialize)]
pub enum DownMsg {
    MessageReceived(Message)
}

// ------ CorId ------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CorId(String);

#[cfg(feature = "frontend")]
impl Default for CorId {
    fn default() -> Self {
        CorId(ulid::generate_ulid_string())
    }
}

// ------ AuthToken ------

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct AuthToken;
