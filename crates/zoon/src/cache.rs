use crate::runtime::CACHES;
use crate::cache_map::{Id, Creator};
use crate::relations::__Relations;
use crate::block_call_stack::__Block;
use std::marker::PhantomData;
use std::any::Any;

pub fn cache<T: 'static>(id: Id, creator: impl FnOnce() -> T + Clone + 'static) -> Cache<T> {
    let id_exists = CACHES.with(|caches| {
        caches.borrow().contains_id(id)
    });

    let creator = Box::new(move || Box::new(Some(creator.clone()())) as Box<dyn Any>);
    let data = creator();

    if !id_exists {
        CACHES.with(move |caches| {
            caches.borrow_mut().insert(id, data, creator);
        });
    }
    Cache::new(id)
}

pub struct Cache<T> {
    pub id: Id,
    phantom_data: PhantomData<T>,
}

impl<T> std::fmt::Debug for Cache<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}

impl<T> Copy for Cache<T> {}
impl<T> Clone for Cache<T> {
    fn clone(&self) -> Cache<T> {
        Cache::<T> {
            id: self.id,
            phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> Cache<T>
where
    T: 'static,
{
    pub(crate) fn new(id: Id) -> Cache<T> {
        Cache {
            id,
            phantom_data: PhantomData,
        }
    }

    pub(crate) fn set(self, data: T, creator: Creator) {
        let data = Box::new(Some(data));
        CACHES.with(|caches| {
            caches
                .borrow_mut()
                .insert(self.id, data, creator)
        });
        __Relations::refresh_dependents(&__Block::Cache(self.id));
    }

    pub(crate) fn remove(self) -> Option<(T, Creator)> {
        CACHES.with(|caches| {
            caches
                .borrow_mut()
                .remove::<T>(self.id)
        })
    }

    pub fn update(self, updater: impl FnOnce(T) -> T) {
        let (data, creator) = self.remove().expect("an cache data with the given id");
        self.set(updater(data), creator);
    }

    pub fn update_mut(self, updater: impl FnOnce(&mut T)) {
        let (mut data, creator) = self.remove().expect("an cache data with the given id");
        updater(&mut data);
        self.set(data, creator);
    }

    pub fn map<U>(self, mapper: impl FnOnce(&T) -> U) -> U {
        CACHES.with(|caches| {
            let cache_map = caches.borrow();
            let data = cache_map.data(self.id)
                .expect("an cache data with the given id");
            mapper(data)
        })
    }

    pub fn map_mut<U>(self, mapper: impl FnOnce(&mut T) -> U) -> U {
        let (mut data, creator) = self.remove().expect("an cache data with the given id");
        let output = mapper(&mut data);
        self.set(data, creator);
        output
    }

    pub fn use_ref<U>(self, user: impl FnOnce(&T)) {
        CACHES.with(|caches| {
            let cache_map = caches.borrow();
            let data = cache_map.data(self.id)
                .expect("an cache data with the given id");
            user(data)
        })
    }
}

pub trait CloneCache<T: Clone + 'static> {
    fn inner(&self) -> T;
}

impl<T: Clone + 'static> CloneCache<T> for Cache<T> {
    fn inner(&self) -> T {
        self.map(Clone::clone)
    }
}
