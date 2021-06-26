use crate::actor::{ActorId, PVar};
use std::borrow::Borrow;

pub trait Index {
    type PVar: PVar;
    type Actor;

    fn insert(&self, _key: <Self::PVar as PVar>::Value, _actor_id: ActorId) {
        todo!()
    }

    fn get(&self, _key: impl Borrow<<Self::PVar as PVar>::Value>) -> Option<Self::Actor> {
        todo!()
    }

    fn for_each(&self, _f: impl FnMut(<Self::PVar as PVar>::Value, Self::Actor)) {
        todo!()
    }
}

