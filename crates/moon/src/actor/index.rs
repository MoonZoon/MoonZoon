use crate::actor::{ActorId, Actor, PVar};
use std::borrow::Borrow;

pub trait Index {
    type PVar: PVar;
    type Actor: Actor;

    fn insert(&self, _key: <Self::PVar as PVar>::Value, _actor_id: ActorId) {
        todo!()
    }

    fn get(&self, _key: impl Borrow<<Self::PVar as PVar>::Value>) -> Option<Self::Actor> {
        todo!()
    }
}

