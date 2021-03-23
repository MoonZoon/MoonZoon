use crate::runtime::CVARS;
use std::marker::PhantomData;
use crate::tracked_call::TrackedCallId;
use crate::relations::__Relations;
use crate::block_call_stack::__Block;
use crate::log;

pub struct CVar<T> {
    pub id: TrackedCallId,
    phantom_data: PhantomData<T>,
}

impl<T> std::fmt::Debug for CVar<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}

impl<T> Copy for CVar<T> {}
impl<T> Clone for CVar<T> {
    fn clone(&self) -> CVar<T> {
        CVar::<T> {
            id: self.id,
            phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> CVar<T>
where
    T: 'static,
{
    pub(crate) fn new(id: TrackedCallId) -> CVar<T> {
        CVar {
            id,
            phantom_data: PhantomData,
        }
    }

    pub fn exists(self) -> bool {
        CVARS.with(|l_vars| {
            l_vars.borrow().contains_id(&self.id)
        })
    }

    pub fn set(self, data: T) {
        CVARS.with(|l_vars| {
            l_vars
                .borrow_mut()
                .insert(self.id, data)
        });
        // log!("SET CVar");
        // __Relations::refresh_dependents(&__Block::CVar(self.id));
    }

    fn remove(self) ->T {
        // log!("REMOVE {:#?}", self.id);
        CVARS.with(|l_vars| {
            l_vars
                .borrow_mut()
                .remove::<T>(&self.id)
        })
    }

    pub fn update(self, updater: impl FnOnce(T) -> T) {
        let data = self.remove();
        self.set(updater(data));
    }

    pub fn update_mut(self, updater: impl FnOnce(&mut T)) {
        let mut data = self.remove();
        updater(&mut data);
        self.set(data);
    }

    pub fn map<U>(self, mapper: impl FnOnce(&T) -> U) -> U {
        CVARS.with(|l_vars| {
            let l_var_map = l_vars.borrow();
            let data = l_var_map.data(&self.id);
            mapper(data)
        })
    }

    pub fn map_mut<U>(self, mapper: impl FnOnce(&mut T) -> U) -> U {
        let mut data = self.remove();
        let output = mapper(&mut data);
        self.set(data);
        output
    }

    pub fn use_ref<U>(self, user: impl FnOnce(&T)) {
        CVARS.with(|l_vars| {
            let l_var_map = l_vars.borrow();
            let data = l_var_map.data(&self.id);
            user(data)
        })
    }
}

pub trait CloneCVar<T: Clone + 'static> {
    fn inner(&self) -> T;
}

impl<T: Clone + 'static> CloneCVar<T> for CVar<T> {
    fn inner(&self) -> T {
        self.map(Clone::clone)
    }
}
