use crate::actor::ActorId;

pub trait PVar: Sized {
    const KEY: &'static str = "session_Id";
    type Value;

    fn actor_id(&self) -> ActorId;

    fn create(self, _value: Self::Value) -> Self {
        todo!()
    }

    fn remove(&self) {
        todo!()
    }
}
