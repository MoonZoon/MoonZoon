use crate::*;
use futures_channel::oneshot;
use moonlight::serde::{de::DeserializeOwned, Serialize};
use moonlight::{serde_json, AuthToken, CorId, SessionId};
use std::{
    collections::BTreeMap,
    error::Error,
    fmt,
    marker::PhantomData,
    pin::Pin,
    sync::{Arc, Mutex},
};
use web_sys::{Request, RequestInit, Response};

mod sse;
use sse::SSE;

// ------ DMsgSenders ------

struct DMsgSenders<DMsg>(Arc<Mutex<BTreeMap<CorId, oneshot::Sender<DMsg>>>>);

impl<DMsg> DMsgSenders<DMsg> {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(BTreeMap::new())))
    }

    fn remove(&self, cor_id: &CorId) -> Option<oneshot::Sender<DMsg>> {
        self.0.lock().unwrap_throw().remove(cor_id)
    }

    fn insert(&self, cor_id: CorId, sender: oneshot::Sender<DMsg>) {
        self.0.lock().unwrap_throw().insert(cor_id, sender);
    }
}

impl<DMsg> Clone for DMsgSenders<DMsg> {
    fn clone(&self) -> Self {
        DMsgSenders(Arc::clone(&self.0))
    }
}

// ------ Connection ------

pub struct Connection<UMsg, DMsg> {
    session_id: SessionId,
    _sse: SSE,
    auth_token_getter:
        Option<Box<dyn Fn() -> Pin<Box<dyn Future<Output = Option<AuthToken>>>> + Send + Sync>>,
    msg_types: PhantomData<(UMsg, DMsg)>,
    d_msg_senders: DMsgSenders<DMsg>,
}

impl<UMsg: Serialize, DMsg: DeserializeOwned + 'static> Connection<UMsg, DMsg> {
    pub fn new(down_msg_handler: impl FnMut(DMsg, CorId) + Send + Sync + 'static) -> Self {
        let d_msg_senders = DMsgSenders::new();

        let down_msg_handler = {
            let d_msg_senders = d_msg_senders.clone();
            let down_msg_handler = Arc::new(Mutex::new(down_msg_handler));

            move |d_msg: DMsg, cor_id: CorId| {
                if let Some(d_msg_sender) = d_msg_senders.remove(&cor_id) {
                    let down_msg_handler = Arc::clone(&down_msg_handler);
                    Task::start(async move {
                        if let Err(d_msg) = d_msg_sender.send(d_msg) {
                            (down_msg_handler.lock().unwrap_throw())(d_msg, cor_id);
                        }
                    });
                } else {
                    (down_msg_handler.lock().unwrap_throw())(d_msg, cor_id)
                }
            }
        };

        let session_id = SessionId::new();
        Self {
            session_id,
            _sse: SSE::new(session_id, down_msg_handler),
            auth_token_getter: None,
            msg_types: PhantomData,
            d_msg_senders,
        }
    }

    pub fn auth_token_getter<IAT, FIAT>(
        mut self,
        getter: impl Fn() -> FIAT + Send + Sync + 'static,
    ) -> Self
    where
        IAT: Into<Option<AuthToken>>,
        FIAT: Future<Output = IAT>,
    {
        let getter = Arc::new(getter);
        self.auth_token_getter = Some(Box::new(move || {
            let getter = Arc::clone(&getter);
            Box::pin(async move { getter().await.into() })
        }));
        self
    }

    pub async fn send_up_msg(&self, up_msg: UMsg) -> Result<CorId, SendUpMsgError> {
        self.send_up_msg_with_options(up_msg, MsgOptions::default())
            .await
    }

    pub async fn send_up_msg_with_options(
        &self,
        up_msg: UMsg,
        msg_options: MsgOptions,
    ) -> Result<CorId, SendUpMsgError> {
        self.send_up_msg_with_cor_id_and_options(up_msg, CorId::new(), msg_options)
            .await
    }

    async fn send_up_msg_with_cor_id_and_options(
        &self,
        up_msg: UMsg,
        cor_id: CorId,
        msg_options: MsgOptions,
    ) -> Result<CorId, SendUpMsgError> {
        // ---- RequestInit ----
        #[cfg(feature = "serde-lite")]
        let body = serde_json::to_string(&up_msg.serialize().unwrap_throw()).unwrap_throw();
        #[cfg(feature = "serde")]
        let body = serde_json::to_string(&up_msg).unwrap_throw();

        let mut request_init = RequestInit::new();
        request_init.method("POST").body(Some(&JsValue::from(body)));

        // ---- Request ----
        let request =
            Request::new_with_str_and_init("/_api/up_msg_handler", &request_init).unwrap_throw();

        // ---- Headers ----
        let headers = request.headers();
        headers
            .set("X-Correlation-ID", &cor_id.to_string())
            .unwrap_throw();
        headers
            .set("X-Session-ID", &self.session_id.to_string())
            .unwrap_throw();

        if msg_options.auth_token {
            let auth_token = if let Some(auth_token_getter) = &self.auth_token_getter {
                auth_token_getter().await
            } else {
                None
            };
            if let Some(auth_token) = auth_token {
                headers
                    .set("X-Auth-Token", auth_token.as_str())
                    .unwrap_throw();
            }
        }

        // ---- Response ----
        let response = JsFuture::from(window().fetch_with_request(&request))
            .await
            .map_err(|error| SendUpMsgError::RequestFailed(error))?
            .unchecked_into::<Response>();

        if response.ok() {
            return Ok(cor_id);
        }
        Err(SendUpMsgError::ResponseIsNot2xx)
    }

    pub async fn exchange_msgs(&self, up_msg: UMsg) -> Result<(DMsg, CorId), ExchangeMsgsError> {
        self.exchange_msgs_with_options(up_msg, MsgOptions::default())
            .await
    }

    pub async fn exchange_msgs_with_options(
        &self,
        up_msg: UMsg,
        msg_options: MsgOptions,
    ) -> Result<(DMsg, CorId), ExchangeMsgsError> {
        let cor_id = CorId::new();
        let (d_msg_sender, d_msg_receiver) = oneshot::channel();

        self.d_msg_senders.insert(cor_id, d_msg_sender);

        self.send_up_msg_with_cor_id_and_options(up_msg, cor_id, msg_options)
            .await
            .map_err(ExchangeMsgsError::SendError)?;
        let d_msg = d_msg_receiver
            .await
            .map_err(|_| ExchangeMsgsError::ReceiveError(ReceiveDownMsgError::ConnectionClosed))?;
        Ok((d_msg, cor_id))
    }
}

// ------ MsgOptions ------

#[derive(Debug, Clone, Copy)]
pub struct MsgOptions {
    auth_token: bool,
}

impl Default for MsgOptions {
    fn default() -> Self {
        Self { auth_token: true }
    }
}

impl MsgOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn auth_token(mut self, include: bool) -> Self {
        self.auth_token = include;
        self
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
            Self::RequestFailed(error) => {
                write!(f, "request failed: {:?}", error)
            }
            Self::ResponseIsNot2xx => {
                write!(f, "response status is not 2xx")
            }
        }
    }
}

impl Error for SendUpMsgError {}

// ------ ReceiveDownMsgError ------

#[derive(Debug)]
pub enum ReceiveDownMsgError {
    ConnectionClosed,
}

impl fmt::Display for ReceiveDownMsgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionClosed => {
                write!(f, "cannot receive DownMsg, connection closed")
            }
        }
    }
}

impl Error for ReceiveDownMsgError {}

// ------ ExchangeMsgsError ------

#[derive(Debug)]
pub enum ExchangeMsgsError {
    SendError(SendUpMsgError),
    ReceiveError(ReceiveDownMsgError),
}

impl fmt::Display for ExchangeMsgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SendError(error) => {
                write!(f, "{error}")
            }
            Self::ReceiveError(error) => {
                write!(f, "{error}")
            }
        }
    }
}

impl Error for ExchangeMsgsError {}
