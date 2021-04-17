use crate::runtime::CMP_VARS;
use std::marker::PhantomData;
use crate::tracked_call::TrackedCallId;
use crate::relations::__Relations;
use crate::block_call_stack::__Block;

pub struct CmpVar<T> {
    pub id: TrackedCallId,
    phantom_data: PhantomData<T>,
}

impl<T> Eq for CmpVar<T> {}
impl<T> PartialEq for CmpVar<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> std::fmt::Debug for CmpVar<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}

impl<T> Copy for CmpVar<T> {}
impl<T> Clone for CmpVar<T> {
    fn clone(&self) -> CmpVar<T> {
        CmpVar::<T> {
            id: self.id,
            phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> CmpVar<T>
where
    T: 'static,
{
    pub(crate) fn new(id: TrackedCallId) -> CmpVar<T> {
        CmpVar {
            id,
            phantom_data: PhantomData,
        }
    }

    pub fn exists(self) -> bool {
        CMP_VARS.with(|cmp_vars| {
            cmp_vars.borrow().contains_id(&self.id)
        })
    }

    pub fn set(self, data: T) {
        CMP_VARS.with(|cmp_vars| {
            cmp_vars
                .borrow_mut()
                .insert(self.id, data)
        });
        // log!("SET CmpVar");
        __Relations::refresh_dependents(&__Block::CmpVar(self.id));
    }

    fn remove(self) ->T {
        // log!("REMOVE {:#?}", self.id);
        CMP_VARS.with(|cmp_vars| {
            cmp_vars
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
        CMP_VARS.with(|cmp_vars| {
            let cmp_var_map = cmp_vars.borrow();
            let data = cmp_var_map.data(&self.id);
            mapper(data)
        })
    }

    pub fn map_mut<U>(self, mapper: impl FnOnce(&mut T) -> U) -> U {
        let mut data = self.remove();
        let output = mapper(&mut data);
        self.set(data);
        output
    }

    pub fn use_ref(self, user: impl FnOnce(&T)) {
        CMP_VARS.with(|cmp_vars| {
            let cmp_var_map = cmp_vars.borrow();
            let data = cmp_var_map.data(&self.id);
            user(data)
        })
    }
}

pub trait CloneCmpVar<T: Clone + 'static> {
    fn inner(&self) -> T;
}

impl<T: Clone + 'static> CloneCmpVar<T> for CmpVar<T> {
    fn inner(&self) -> T {
        self.map(Clone::clone)
    }
}
