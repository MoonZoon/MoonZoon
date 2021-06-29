use moonlight::{AuthToken, CorId, SessionId};

#[derive(Debug)]
pub struct UpMsgRequest<UMsg> {
    pub up_msg: UMsg,
    pub session_id: SessionId,
    pub cor_id: CorId,
    pub auth_token: Option<AuthToken>,
}
