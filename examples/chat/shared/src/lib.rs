use serde_lite::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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
