use crate::actor::{ActorId, Actor, PVar};
use std::borrow::Borrow;

pub trait Index {
    type PVar: PVar;
    type Actor: Actor;

    fn insert(&self, key: <Self::PVar as PVar>::Value, actor_id: ActorId) {
        todo!()
    }

    fn get(&self, key: impl Borrow<<Self::PVar as PVar>::Value>) -> Option<Self::Actor> {
        todo!()
    }
}

