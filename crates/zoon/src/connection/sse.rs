use crate::moonlight::{SessionId, CorId, DownMsgTransporterForDe};
pub struct SSE<DMsg> {
    down_msg_handler: Box<dyn Fn(DMsg, CorId) + Send + Sync>,
}

impl<DMsg> SSE<DMsg> {
    pub fn new(session_id: SessionId, down_msg_handler: impl FnOnce(DMsg, CorId) + Clone + Send + Sync + 'static) -> Self {
        let down_msg_handler = move |down_msg, cor_id| (down_msg_handler.clone())(down_msg, cor_id);
        Self {
            down_msg_handler: Box::new(down_msg_handler),
        }
    }

    fn handle_down_msg_transporter() {
        // DownMsgTransporterForDe

        // UMsg::deserialize(
        //     &serde_json::from_slice(&body).map_err(error::JsonPayloadError::Deserialize)?
        // ).map_err(error::ErrorBadRequest)
    }
} 
