pub mod sessions;
pub mod index;
pub mod p_var;

pub use index::Index;
pub use p_var::PVar;

// ------ ActorId ------

#[derive(Debug, Clone, Copy)]
pub struct ActorId;

impl ActorId {
    pub(crate) fn new() -> Self {
        Self
    }
}

// ------ Actor ------

pub trait Actor {
    const KEY: &'static str;

    fn new_actor_id() -> ActorId {
        ActorId::new()
    }
}
