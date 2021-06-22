use moonlight::{CorId, AuthToken};

#[derive(Debug)]
pub struct UpMsgRequest<UMsg> {
    pub up_msg: UMsg,
    pub cor_id: CorId,
    pub auth_token: Option<AuthToken>,
}
