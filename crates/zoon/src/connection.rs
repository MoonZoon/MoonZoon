use crate::*;
use std::marker::PhantomData;
use moonlight::{CorId, AuthToken, serde_lite::{Serialize, Deserialize}, serde_json, SessionId};
use web_sys::{Request, RequestInit, Response};
use std::error::Error;
use std::fmt;

// ------ Connection ------

pub struct Connection<UMsg, DMsg> {
    session_id: SessionId,
    down_msg_handler: Option<Box<dyn Fn(DMsg, CorId) + Send + Sync>>,
    auth_token_getter: Option<Box<dyn Fn() -> Option<AuthToken> + Send + Sync>>,
    msg_types: PhantomData<(UMsg, DMsg)>,
}

impl<UMsg: Serialize, DMsg: Deserialize> Connection<UMsg, DMsg> {
    pub fn new() -> Self {
        Connection {
            session_id: SessionId::new(),
            down_msg_handler: None,
            auth_token_getter: None,
            msg_types: PhantomData
        }
    }

    pub fn down_msg_handler(mut self, handler: impl FnOnce(DMsg, CorId) + Clone + Send + Sync + 'static) -> Self {
        let handler = move |down_msg, cor_id| (handler.clone())(down_msg, cor_id);
        self.down_msg_handler = Some(Box::new(handler));
        self
    }

    pub fn auth_token_getter<IAT>(mut self, getter: impl FnOnce() -> IAT + Clone + Send + Sync + 'static) -> Self 
        where IAT: Into<Option<AuthToken>>
    {
        let getter = move || (getter.clone())().into();
        self.auth_token_getter = Some(Box::new(getter));
        self
    }

    pub async fn send_up_msg(&self, up_msg: UMsg) -> Result<(), SendUpMsgError> {
        // ---- RequestInit ----
        let body = serde_json::to_string(&up_msg.serialize().unwrap_throw()).unwrap_throw();

        let mut request_init = RequestInit::new();
        request_init
            .method("POST")
            .body(Some(&JsValue::from(body)));

        // ---- Request ----
        let request = Request::new_with_str_and_init("_api/up_msg_handler", &request_init).unwrap_throw();

        // ---- Headers ----
        let headers = request.headers();
        headers.set("X-Correlation-ID", &CorId::new().to_string()).unwrap_throw();
        headers.set("X-Session-ID", &self.session_id.to_string()).unwrap_throw();

        let auth_token = self
            .auth_token_getter
            .as_ref()
            .and_then(|auth_token_getter| auth_token_getter());
        if let Some(auth_token) = auth_token {
            headers.set("X-Auth-Token", auth_token.as_str()).unwrap_throw();
        }

        // ---- Response ----
        let response = JsFuture::from(window().fetch_with_request(&request))
            .await
            .map_err(|error| SendUpMsgError::RequestFailed(error))?
            .unchecked_into::<Response>();

        if response.ok() {
            return Ok(())
        }
        Err(SendUpMsgError::ResponseIsNot2xx)
    }
}

// ------ SendUpMsgError ------

#[derive(Debug)]
pub enum SendUpMsgError {
    RequestFailed(JsValue),
    ResponseIsNot2xx,
}

impl fmt::Display for SendUpMsgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SendUpMsgError::RequestFailed(error) => {
                write!(f, "request failed: {:?}", error)
            }
            SendUpMsgError::ResponseIsNot2xx => {
                write!(f, "response status is not 2xxx")
            }
        }
    }
}

impl Error for SendUpMsgError {}


