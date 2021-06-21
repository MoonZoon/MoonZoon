use moonlight::serde_lite::{self, Deserialize, Serialize};

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
