use moonlight::*;

// ------ UpMsg ------

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum UpMsg {
    SendMessage(Message),
}

// ------ DownMsg ------

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum DownMsg {
    MessageReceived(Message),
}

// ------ Message ------

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "serde")]
pub struct Message {
    pub username: String,
    pub text: String,
}
