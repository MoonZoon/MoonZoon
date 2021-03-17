use std::collections::HashMap;
use std::any::Any;

pub type Id = u128;
pub type Creator = Box<dyn Fn() -> Box<dyn Any>>;

pub(crate) struct CacheMap {
    caches: HashMap<Id, CacheMapValue>,
}

struct CacheMapValue {
    data: Box<dyn Any>,
    creator: Creator,
}

impl CacheMap {
    pub(crate) fn new() -> Self {
        Self {
            caches: HashMap::new(),
        }
    }

    pub(crate) fn data<T: 'static>(&self, id: Id) -> Option<&T> {
        self
            .caches
            .get(&id)?
            .data
            .downcast_ref::<Option<T>>()?
            .as_ref()
    }

    pub(crate) fn insert(&mut self, id: Id, data: Box<dyn Any>, creator: Creator) {
        self
            .caches
            .insert(id, CacheMapValue { 
                data, 
                creator,
            });
    }

    pub(crate) fn remove<T: 'static>(&mut self, id: Id) -> Option<(T, Creator)> {
        let CacheMapValue { mut data, creator } = self.caches.remove(&id)?;
        let data = data.downcast_mut::<Option<T>>()?.take()?;
        Some((data, creator))
    }

    pub(crate) fn remove_return_creator(&mut self, id: Id) -> Option<Creator> {
        let CacheMapValue { creator, .. } = self.caches.remove(&id)?;
        Some(creator)
    }

    pub(crate) fn contains_id(&self, id: Id) -> bool {
        self
            .caches
            .contains_key(&id)
    }
}
