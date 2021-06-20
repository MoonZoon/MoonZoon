use std::marker::PhantomData;

pub struct Connection<UMsg, DMsg, CId, ATok> {
    msg_types: PhantomData<(UMsg, DMsg, CId, ATok)>,
}

impl<UMsg, DMsg, CId, ATok> Connection<UMsg, DMsg, CId, ATok> {
    pub fn new(handler: impl FnOnce(DMsg, CId) + Clone) -> Self {
        Connection {
            msg_types: PhantomData
        }
    }

    pub fn send_up_msg(&self, up_msg: UMsg, token: Option<ATok>) {

    }
}


