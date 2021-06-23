use crate::actor::ActorId;

pub trait PVar: Default {
    const KEY: &'static str = "session_Id";
    type Value;

    fn create(_value: Self::Value, _actor_id: ActorId) -> Self {
        todo!()
    }

    fn remove(_actor_id: ActorId) {
        todo!()
    }
}
