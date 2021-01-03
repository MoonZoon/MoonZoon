use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub username: String,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub enum UpMsg {
    SendMessage(Message),
}

#[derive(Serialize, Deserialize)]
pub enum DownMsg {
    MessageReceived(Message)
}
