use std::collections::HashMap;
use std::any::Any;

pub type Id = u128;

pub(crate) struct CacheMap {
    caches: HashMap<Id, CacheMapValue>,
}

struct CacheMapValue {
    data: Box<dyn Any>,
    creator: Box<dyn Fn() -> Box<dyn Any>>,
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

    pub(crate) fn insert(&mut self, id: Id, data: Box<dyn Any>, creator: Box<dyn Fn() -> Box<dyn Any>>) {
        self
            .caches
            .insert(id, CacheMapValue { 
                data, 
                creator,
            });
    }

    pub(crate) fn contains_id(&self, id: Id) -> bool {
        self
            .caches
            .contains_key(&id)
    }
}
