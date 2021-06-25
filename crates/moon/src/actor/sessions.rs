use moonlight::{SessionId, CorId};
use crate::actor::{Actor, ActorId, Index, PVar};
use futures::future::join_all;
use apply::Apply;
use std::borrow::Borrow;

// @TODO rewrite to a proper virtual actor

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

    // --

    fn insert(&self, _key: <Self::PVar as PVar>::Value, _actor_id: ActorId) {

    }

    fn get(&self, _key: impl Borrow<<Self::PVar as PVar>::Value>) -> Option<Self::Actor> {
        None
    }
}

impl Iterator for BySessionId {
    type Item = (SessionId, SessionActor);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// ------ PVars ------

#[derive(Clone, Copy)]
pub struct PVarSessionId(ActorId);
impl PVar for PVarSessionId {
    const KEY: &'static str = "session_id";
    type Value = SessionId;

    fn actor_id(&self) -> ActorId {
        self.0
    }

    // --

    fn create(self, _value: Self::Value) -> Self {
        self
    }

    fn remove(&self) {

    }
}

// ------ Actor ------

pub struct SessionActor {
    actor_id: ActorId,   
    pub session_id: PVarSessionId,
}

impl Actor for SessionActor {
    const KEY: &'static str = "_session";

    fn actor_id(&self) -> ActorId {
        self.actor_id
    }

    fn revive(actor_id: ActorId) -> Self {
        Self {
            actor_id,
            session_id: PVarSessionId(actor_id),
        } 
    }

    fn remove(&self) {
        self.session_id.remove();
    }
}

impl SessionActor {
    pub fn create(session_id: SessionId) -> Self {
        let actor_id = Self::new_actor_id();

        by_session_id().insert(session_id, actor_id);

        Self {
            actor_id,
            session_id: PVarSessionId(actor_id).create(session_id),
        }
    }

    pub async fn send_down_msg<DMsg>(&self, _down_msg: &DMsg, _cor_id: CorId) {

    }
}


