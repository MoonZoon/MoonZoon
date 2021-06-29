use crate::actor::{ActorId, ActorInstance, Index, PVar};
use crate::sse::ShareableSSEMethods;
use crate::MessageSSE;
use chashmap::CHashMap;
use futures::future::join_all;
use moonlight::{serde_json, CorId, DownMsgTransporterForSer, Serialize, SessionId};
use once_cell::sync::Lazy;
use std::borrow::Borrow;
use std::cell::RefCell;

// @TODO rewrite to a proper virtual actor

pub async fn broadcast_down_msg<DMsg: Serialize>(down_msg: &DMsg, cor_id: CorId) {
    let mut send_down_msg_futs = vec![];
    by_session_id().for_each(|_, session_actor| {
        send_down_msg_futs.push(async move { session_actor.send_down_msg(down_msg, cor_id).await });
    });
    join_all(send_down_msg_futs).await;
}

// ------ Indices ------

static BY_SESSION_ID: Lazy<CHashMap<SessionId, SessionActor>> = Lazy::new(CHashMap::new);

pub const fn by_session_id() -> BySessionId {
    BySessionId
}
pub struct BySessionId;
impl Index for BySessionId {
    type PVar = PVarSessionId;
    type Actor = SessionActor;

    // --

    fn insert(&self, key: <Self::PVar as PVar>::Value, actor_id: ActorId) {
        BY_SESSION_ID.insert(key, SessionActor { actor_id });
    }

    fn get(&self, key: impl Borrow<<Self::PVar as PVar>::Value>) -> Option<Self::Actor> {
        BY_SESSION_ID
            .get(key.borrow())
            .map(|session_actor| *session_actor)
    }

    fn for_each(&self, f: impl FnMut(SessionId, SessionActor)) {
        let f = RefCell::new(f);
        BY_SESSION_ID.retain(|session_id, session_actor| {
            f.borrow_mut()(*session_id, *session_actor);
            true
        });
    }
}

// ------ PVars ------

static PVAR_SESSION_IDS: Lazy<CHashMap<ActorId, SessionId>> = Lazy::new(CHashMap::new);

#[derive(Clone, Copy)]
pub struct PVarSessionId(ActorId);
impl PVar for PVarSessionId {
    const KEY: &'static str = "session_id";
    type Value = SessionId;

    fn actor_id(&self) -> ActorId {
        self.0
    }

    // --

    fn create(self, value: Self::Value) -> Self {
        PVAR_SESSION_IDS.insert(self.0, value);
        self
    }

    fn read(&self) -> Option<Self::Value> {
        PVAR_SESSION_IDS
            .get(&self.0)
            .map(|session_id| session_id.clone())
    }

    fn write(&self, value: Self::Value) {
        PVAR_SESSION_IDS.insert(self.0, value);
    }

    fn remove(&self) {
        PVAR_SESSION_IDS.remove(&self.0);
    }
}

// ------ Actor ------

// -- SessionActor --

#[derive(Clone, Copy)]
pub struct SessionActor {
    actor_id: ActorId,
}

impl SessionActor {
    pub fn create(session_id: SessionId, message_sse: MessageSSE) -> Self {
        Self {
            actor_id: SessionActorInstance::create(session_id, message_sse),
        }
    }

    pub(crate) fn remove(&self) {
        if let Some(instance) = SESSION_ACTOR_INSTANCES.remove(&self.actor_id) {
            instance.remove()
        }
    }

    pub async fn send_down_msg<DMsg: Serialize>(&self, down_msg: &DMsg, cor_id: CorId) {
        if let Some(instance) = SESSION_ACTOR_INSTANCES.get(&self.actor_id) {
            instance.send_down_msg(down_msg, cor_id).await;
        }
    }
}

// -- SessionActorInstance --

static SESSION_ACTOR_INSTANCES: Lazy<CHashMap<ActorId, SessionActorInstance>> =
    Lazy::new(CHashMap::new);

struct SessionActorInstance {
    actor_id: ActorId,
    message_sse: MessageSSE,
    session_id: PVarSessionId,
}

impl ActorInstance for SessionActorInstance {
    const KEY: &'static str = "_session";

    fn actor_id(&self) -> ActorId {
        self.actor_id
    }

    fn revive(_actor_id: ActorId) -> Self {
        unimplemented!("revive not implemented for SessionActorInstance");
    }

    fn remove(&self) {
        self.session_id.remove();
        SESSION_ACTOR_INSTANCES.remove(&self.actor_id);
    }
}

impl SessionActorInstance {
    fn create(session_id: SessionId, message_sse: MessageSSE) -> ActorId {
        let actor_id = ActorId::new();

        by_session_id().insert(session_id, actor_id);

        let actor_instance = Self {
            actor_id,
            message_sse,
            session_id: PVarSessionId(actor_id).create(session_id),
        };
        SESSION_ACTOR_INSTANCES.insert(actor_id, actor_instance);
        actor_id
    }

    pub async fn send_down_msg<DMsg: Serialize>(&self, down_msg: &DMsg, cor_id: CorId) {
        let session_id = self.session_id.read().unwrap();

        let down_msg_transporter = DownMsgTransporterForSer { down_msg, cor_id };
        let down_msg_transporter =
            serde_json::to_string(&down_msg_transporter.serialize().unwrap()).unwrap();

        self.message_sse
            .send(&session_id, "down_msg", &down_msg_transporter);
    }
}
