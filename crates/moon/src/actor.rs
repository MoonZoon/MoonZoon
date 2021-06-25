pub mod sessions;
pub mod index;
pub mod p_var;

pub use index::Index;
pub use p_var::PVar;
use uuid::Uuid;

// ------ ActorId ------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub struct ActorId(Uuid);

impl ActorId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

// ------ ActorInstance ------

pub trait ActorInstance {
    const KEY: &'static str;

    fn actor_id(&self) -> ActorId;

    fn revive(actor_id: ActorId) -> Self;

    fn remove(&self);
}
