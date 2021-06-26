use moonlight::{SessionId, CorId, DownMsgTransporterForSer, serde_json, Serialize};
use crate::actor::{ActorInstance, ActorId, Index, PVar};
use crate::MessageSSE;
use crate::sse::ShareableSSEMethods;
use futures::future::join_all;
use std::{borrow::Borrow, collections::HashMap};
use parking_lot::Mutex;
use once_cell::sync::Lazy;

// @TODO rewrite to a proper virtual actor

pub async fn broadcast_down_msg<DMsg: Serialize>(down_msg: &DMsg, cor_id: CorId) {
    let mut send_down_msg_futs = vec![];
    by_session_id().for_each(|_, session_actor| {
        send_down_msg_futs.push(async move {
            session_actor.send_down_msg(down_msg, cor_id).await
        });
    });
    join_all(send_down_msg_futs).await;
}

// ------ Indices ------

static BY_SESSION_ID: Lazy<Mutex<HashMap<SessionId, SessionActor>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub const fn by_session_id() -> BySessionId { 
    BySessionId
}
pub struct BySessionId;
impl Index for BySessionId {
    type PVar = PVarSessionId;
    type Actor = SessionActor;

    // --

    fn insert(&self, key: <Self::PVar as PVar>::Value, actor_id: ActorId) {
        BY_SESSION_ID.lock().insert(key, SessionActor { actor_id });
    }

    fn get(&self, key: impl Borrow<<Self::PVar as PVar>::Value>) -> Option<Self::Actor> {
        BY_SESSION_ID
            .lock()
            .get(key.borrow())
            .copied()
    }

    fn for_each(&self, mut f: impl FnMut(SessionId, SessionActor)) {
        BY_SESSION_ID
            .lock()
            .iter()
            .for_each(|(session_id, session_actor)| {
                f(*session_id, *session_actor)
            });
    }
}

// ------ PVars ------

static PVAR_SESSION_IDS: Lazy<Mutex<HashMap<ActorId, SessionId>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

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
        PVAR_SESSION_IDS.lock().insert(self.0, value);
        self
    }

    fn read(&self) -> Option<Self::Value> {
        PVAR_SESSION_IDS.lock().get(&self.0).cloned()
    }

    fn write(&self, value: Self::Value) {
        PVAR_SESSION_IDS.lock().insert(self.0, value);
    }

    fn remove(&self) {
        PVAR_SESSION_IDS.lock().remove(&self.0);
    }
}

// ------ Actor ------

// -- SessionActor --

#[derive(Clone, Copy)]
pub struct SessionActor {
    actor_id: ActorId
}

impl SessionActor {
    pub fn create(session_id: SessionId, message_sse: MessageSSE) -> Self {
        Self {
            actor_id: SessionActorInstance::create(session_id, message_sse)
        }
    }

    pub(crate) fn remove(&self) {
        if let Some(instance) = SESSION_ACTOR_INSTANCES.lock().remove(&self.actor_id) {
            instance.remove();
        }
    }

    pub async fn send_down_msg<DMsg: Serialize>(&self, down_msg: &DMsg, cor_id: CorId) {
        let instances = SESSION_ACTOR_INSTANCES.lock();
        if let Some(instance) = instances.get(&self.actor_id) {
            instance.send_down_msg(down_msg, cor_id).await;
        }
    }
}

// -- SessionActorInstance --

static SESSION_ACTOR_INSTANCES: Lazy<Mutex<HashMap<ActorId, SessionActorInstance>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

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
        SESSION_ACTOR_INSTANCES.lock().remove(&self.actor_id);
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
        SESSION_ACTOR_INSTANCES.lock().insert(actor_id, actor_instance);
        actor_id
    }

    pub async fn send_down_msg<DMsg: Serialize>(&self, down_msg: &DMsg, cor_id: CorId) {
        let session_id = self.session_id.read().unwrap();

        let down_msg_transporter = DownMsgTransporterForSer {
            down_msg,
            cor_id
        };
        let down_msg_transporter = serde_json::to_string(&down_msg_transporter.serialize().unwrap()).unwrap();

        self.message_sse.send(&session_id, "down_msg", &down_msg_transporter);
    }
}


