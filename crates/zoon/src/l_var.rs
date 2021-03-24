use crate::runtime::LVARS;
use std::marker::PhantomData;
use crate::tracked_call::TrackedCallId;
use crate::relations::__Relations;
use crate::block_call_stack::__Block;
use crate::log;

pub struct LVar<T> {
    pub id: TrackedCallId,
    phantom_data: PhantomData<T>,
}

impl<T> std::fmt::Debug for LVar<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}

impl<T> Copy for LVar<T> {}
impl<T> Clone for LVar<T> {
    fn clone(&self) -> LVar<T> {
        LVar::<T> {
            id: self.id,
            phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> LVar<T>
where
    T: 'static,
{
    pub(crate) fn new(id: TrackedCallId) -> LVar<T> {
        LVar {
            id,
            phantom_data: PhantomData,
        }
    }

    pub fn exists(self) -> bool {
        LVARS.with(|l_vars| {
            l_vars.borrow().contains_id(&self.id)
        })
    }

    pub fn set(self, data: T) {
        LVARS.with(|l_vars| {
            l_vars
                .borrow_mut()
                .insert(self.id, data)
        });
        // log!("SET LVar");
        __Relations::refresh_dependents(&__Block::LVar(self.id));
    }

    fn remove(self) ->T {
        // log!("REMOVE {:#?}", self.id);
        LVARS.with(|l_vars| {
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
        LVARS.with(|l_vars| {
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
        LVARS.with(|l_vars| {
            let l_var_map = l_vars.borrow();
            let data = l_var_map.data(&self.id);
            user(data)
        })
    }
}

pub trait CloneLVar<T: Clone + 'static> {
    fn inner(&self) -> T;
}

impl<T: Clone + 'static> CloneLVar<T> for LVar<T> {
    fn inner(&self) -> T {
        self.map(Clone::clone)
    }
}
