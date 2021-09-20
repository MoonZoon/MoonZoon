use crate::actor::{ActorId, PVar};
use std::borrow::Borrow;
use async_trait::async_trait;

#[async_trait(?Send)]
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

    async fn wait_for(&self, key: impl Borrow<<Self::PVar as PVar>::Value> + 'static) -> Option<Self::Actor> {
        // @TODO backoff + jitter + queue or something else?
        let key = key.borrow();
        for i in 0..10 {
            let session = self.get(key);
            if session.is_some() {
                return session
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(i * 200)).await;
        }
        None
    }
}
