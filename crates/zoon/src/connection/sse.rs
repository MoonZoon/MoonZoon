use crate::*;
use crate::moonlight::{SessionId, CorId, DownMsgTransporterForDe};

// ------ SSE ------

pub struct SSE {
    // down_msg_handler: Box<dyn Fn(DMsg, CorId) + Send + Sync>,
    reconnecting_event_source: SendWrapper<ReconnectingEventSource>,
}

impl Drop for SSE {
    fn drop(&mut self) {
        self.reconnecting_event_source.close()
    }
}

impl SSE {
    pub fn new<DMsg>(session_id: SessionId, down_msg_handler: impl FnOnce(DMsg, CorId) + Clone + Send + Sync + 'static) -> Self {
        let down_msg_handler = move |down_msg, cor_id| (down_msg_handler.clone())(down_msg, cor_id);

        let reconnecting_event_source = ReconnectingEventSource::new("/_api/message_sse", None);

        Self {
            // down_msg_handler: Box::new(down_msg_handler),
            reconnecting_event_source: SendWrapper::new(reconnecting_event_source),
        }
    }

    fn handle_down_msg_transporter() {
        // DownMsgTransporterForDe

        // UMsg::deserialize(
        //     &serde_json::from_slice(&body).map_err(error::JsonPayloadError::Deserialize)?
        // ).map_err(error::ErrorBadRequest)
    }
} 

// ------ ReconnectingEventSource ------

#[wasm_bindgen]
extern "C" {
    type ReconnectingEventSource;

    #[wasm_bindgen(constructor)]
    fn new(url: &str, options: Option<ReconnectingEventSourceOptions>) -> ReconnectingEventSource;

    #[wasm_bindgen(method)]
    fn close(this: &ReconnectingEventSource);
}

#[wasm_bindgen]
struct ReconnectingEventSourceOptions {
    // #[wasm_bindgen(js_name = withCredentials)]
    withCredentials: bool,

    max_retry_time: u32,
}

// // https://github.com/fanout/reconnecting-eventsource
// var sse = new ReconnectingEventSource('/_api/reload_sse', {
//     withCredentials: false,
//     max_retry_time: 5000,
// });
// var backendBuildId = null;
// sse.addEventListener("backend_build_id", function (msg) {
//     var newBackendBuildId = msg.data;
//     if (backendBuildId === null) {
//         backendBuildId = newBackendBuildId;
//     } else if (backendBuildId !== newBackendBuildId) {
//         sse.close();
//         location.reload();
//     }
// });
// sse.addEventListener("reload", function (msg) {
//     sse.close();
//     location.reload();
// });
