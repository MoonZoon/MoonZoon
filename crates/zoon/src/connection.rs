use std::marker::PhantomData;
use moonlight::{CorId, AuthToken};

pub struct Connection<UMsg, DMsg> {
    down_msg_handler: Option<Box<dyn Fn(DMsg, CorId) + Send + Sync>>,
    auth_token_getter: Option<Box<dyn Fn() -> Option<AuthToken> + Send + Sync>>,
    msg_types: PhantomData<(UMsg, DMsg)>,
}

impl<UMsg, DMsg> Connection<UMsg, DMsg> {
    pub fn new() -> Self {
        Connection {
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

    pub fn send_up_msg(&self, up_msg: UMsg) {

    }
}


