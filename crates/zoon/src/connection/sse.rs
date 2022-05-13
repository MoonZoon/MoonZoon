use crate::moonlight::{serde_json, CorId, DeserializeOwned, DownMsgTransporterForDe, SessionId};
use crate::{format, *};
use std::{error::Error, fmt};

#[cfg(feature = "moonlight/serde_with_serde")]
use moonlight::serde::{self, Deserialize};

// ------ SSE ------

pub struct SSE {
    reconnecting_event_source: SendWrapper<ReconnectingEventSource>,
    _down_msg_handler: SendWrapper<Closure<dyn Fn(JsValue)>>,
}

impl Drop for SSE {
    fn drop(&mut self) {
        self.reconnecting_event_source.close();
    }
}

impl SSE {
    #[cfg(feature = "serde")]
    pub fn new<DMsg: DeserializeOwned>(
        session_id: SessionId,
        down_msg_handler: impl FnMut(DMsg, CorId) + 'static,
    ) -> Self {
        let down_msg_handler = down_msg_handler_closure(down_msg_handler);

        let reconnecting_event_source = connect(session_id);
        reconnecting_event_source
            .add_event_listener("down_msg", down_msg_handler.as_ref().unchecked_ref());

        Self {
            reconnecting_event_source: SendWrapper::new(reconnecting_event_source),
            _down_msg_handler: SendWrapper::new(down_msg_handler),
        }
    }
    #[cfg(feature = "serde-lite")]
    pub fn new<DMsg: Deserialize>(
        session_id: SessionId,
        down_msg_handler: impl FnMut(DMsg, CorId) + 'static,
    ) -> Self {
        let down_msg_handler = down_msg_handler_closure(down_msg_handler);

        let reconnecting_event_source = connect(session_id);
        reconnecting_event_source
            .add_event_listener("down_msg", down_msg_handler.as_ref().unchecked_ref());

        Self {
            reconnecting_event_source: SendWrapper::new(reconnecting_event_source),
            _down_msg_handler: SendWrapper::new(down_msg_handler),
        }
    }
}

#[cfg(feature = "serde")]
fn down_msg_handler_closure<DMsg: DeserializeOwned>(
    down_msg_handler: impl FnMut(DMsg, CorId) + 'static,
) -> Closure<dyn Fn(JsValue)> {
    let down_msg_handler = move |down_msg, cor_id| (down_msg_handler.clone())(down_msg, cor_id);
    Closure::wrap(Box::new(
        move |event: JsValue| match down_msg_transporter_from_event(event) {
            Ok(DownMsgTransporterForDe { down_msg, cor_id }) => down_msg_handler(down_msg, cor_id),
            Err(error) => crate::eprintln!("{:?}", error),
        },
    ) as Box<dyn Fn(JsValue)>)
}
#[cfg(feature = "serde-lite")]
fn down_msg_handler_closure<DMsg: Deserialize>(
    down_msg_handler: impl FnMut(DMsg, CorId) + 'static,
) -> Closure<dyn Fn(JsValue)> {
    let down_msg_handler = move |down_msg, cor_id| (down_msg_handler.clone())(down_msg, cor_id);
    Closure::wrap(Box::new(
        move |event: JsValue| match down_msg_transporter_from_event(event) {
            Ok(DownMsgTransporterForDe { down_msg, cor_id }) => down_msg_handler(down_msg, cor_id),
            Err(error) => crate::eprintln!("{:?}", error),
        },
    ) as Box<dyn Fn(JsValue)>)
}

#[cfg(feature = "serde")]
fn down_msg_transporter_from_event<DMsg: DeserializeOwned>(
    event: JsValue,
) -> Result<DownMsgTransporterForDe<DMsg>, DownMsgError> {
    let down_msg_transporter = Reflect::get(&event, &JsValue::from("data"))
        .unwrap()
        .as_string()
        .ok_or(DownMsgError::InvalidDataValue)?;

    serde_json::from_str(&down_msg_transporter).map_err(DownMsgError::JsonDeserializationFailed)
}
#[cfg(feature = "serde-lite")]
fn down_msg_transporter_from_event<DMsg: Deserialize>(
    event: JsValue,
) -> Result<DownMsgTransporterForDe<DMsg>, DownMsgError> {
    let down_msg_transporter = Reflect::get(&event, &JsValue::from("data"))
        .unwrap()
        .as_string()
        .ok_or(DownMsgError::InvalidDataValue)?;

    DownMsgTransporterForDe::deserialize(
        &serde_json::from_str(&down_msg_transporter)
            .map_err(DownMsgError::JsonDeserializationFailed)?,
    )
    .map_err(DownMsgError::DeserializationFailed)
}

fn connect(session_id: SessionId) -> ReconnectingEventSource {
    ReconnectingEventSource::new(
        &format!("/_api/message_sse/{}", session_id),
        Some(ReconnectingEventSourceOptions {
            withCredentials: false,
            max_retry_time: 5000,
        }),
    )
}

// ------ DownMsgError ------

#[derive(Debug)]
enum DownMsgError {
    InvalidDataValue,
    JsonDeserializationFailed(serde_json::Error),
    #[cfg(feature = "serde-lite")]
    DeserializationFailed(serde_lite::Error),
}

impl fmt::Display for DownMsgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DownMsgError::InvalidDataValue => {
                write!(f, "invalid DownMsg data value")
            }
            DownMsgError::JsonDeserializationFailed(error) => {
                write!(
                    f,
                    "failed to JSON deserialize DownMsgTransporter: {:?}",
                    error
                )
            }
            #[cfg(feature = "serde-lite")]
            DownMsgError::DeserializationFailed(error) => {
                write!(f, "failed to deserialize DownMsgTransporter: {:?}", error)
            }
        }
    }
}

impl Error for DownMsgError {}

// ------ ReconnectingEventSource ------

#[wasm_bindgen]
extern "C" {
    type ReconnectingEventSource;

    #[wasm_bindgen(constructor)]
    fn new(url: &str, options: Option<ReconnectingEventSourceOptions>) -> ReconnectingEventSource;

    #[wasm_bindgen(method, js_name = addEventListener)]
    fn add_event_listener(this: &ReconnectingEventSource, type_: &str, listener: &js_sys::Function);

    #[wasm_bindgen(method)]
    fn close(this: &ReconnectingEventSource);
}

#[wasm_bindgen]
#[allow(non_snake_case)]
#[allow(dead_code)]
struct ReconnectingEventSourceOptions {
    withCredentials: bool,
    max_retry_time: u32,
}
