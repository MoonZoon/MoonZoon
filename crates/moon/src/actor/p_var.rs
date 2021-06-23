use crate::actor::ActorId;

pub trait PVar: Default {
    const KEY: &'static str = "session_Id";
    type Value;

    fn create(value: Self::Value, actor_id: ActorId) -> Self {
        todo!()
    }

    fn remove(actor_id: ActorId) {
        todo!()
    }
}
