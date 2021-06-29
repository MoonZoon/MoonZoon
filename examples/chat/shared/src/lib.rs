use moonlight::serde_lite::{self, Deserialize, Serialize};

// ------ UpMsg ------

#[derive(Serialize, Deserialize, Debug)]
pub enum UpMsg {
    SendMessage(Message),
}

// ------ DownMsg ------

#[derive(Serialize, Deserialize, Debug)]
pub enum DownMsg {
    MessageReceived(Message),
}

// ------ Message ------

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub username: String,
    pub text: String,
}
