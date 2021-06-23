use moonlight::{SessionId, CorId};
use crate::actor::{Actor, Index, PVar};
use futures::future::join_all;
use apply::Apply;

pub async fn broadcast_down_msg<DMsg>(down_msg: &DMsg, cor_id: CorId) {
    by_session_id()
        .into_iter()
        .map(|(_, session_actor)| {
            async move { session_actor.send_down_msg(down_msg, cor_id).await }
        })
        .apply(join_all)
        .await;
}

// ------ Indices ------

pub const fn by_session_id() -> BySessionId { 
    BySessionId
}
pub struct BySessionId;
impl Index for BySessionId {
    type PVar = PVarSessionId;
    type Actor = SessionActor;
}

impl Iterator for BySessionId {
    type Item = (SessionId, SessionActor);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// ------ PVars ------

#[derive(Default, Clone, Copy)]
pub struct PVarSessionId;
impl PVar for PVarSessionId {
    const KEY: &'static str = "session_id";
    type Value = SessionId;
}

// ------ Actor ------

#[derive(Default)]
pub struct SessionActor {
    pub session_id: PVarSessionId,
}

impl Actor for SessionActor {
    const KEY: &'static str = "_session";
}

impl SessionActor {
    pub fn create(session_id: SessionId) -> Self {
        let actor_id = Self::new_actor_id();

        by_session_id().insert(session_id, actor_id);

        Self {
            session_id: PVarSessionId::create(session_id, actor_id),
        }
    }

    pub async fn send_down_msg<DMsg>(&self, _down_msg: &DMsg, _cor_id: CorId) {

    }
}


