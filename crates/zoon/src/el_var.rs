use crate::runtime::EL_VARS;
use std::marker::PhantomData;
use crate::tracked_call::TrackedCallId;

pub struct ElVar<T> {
    pub id: TrackedCallId,
    phantom_data: PhantomData<T>,
}

impl<T> std::fmt::Debug for ElVar<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}

impl<T> Copy for ElVar<T> {}
impl<T> Clone for ElVar<T> {
    fn clone(&self) -> ElVar<T> {
        ElVar::<T> {
            id: self.id,
            phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> ElVar<T>
where
    T: 'static,
{
    pub(crate) fn new(id: TrackedCallId) -> ElVar<T> {
        ElVar {
            id,
            phantom_data: PhantomData,
        }
    }

    pub fn exists(self) -> bool {
        EL_VARS.with(|el_vars| {
            el_vars.borrow().contains_id(&self.id)
        })
    }

    pub fn set(self, data: T) {
        EL_VARS.with(|el_vars| {
            el_vars
                .borrow_mut()
                .insert(self.id, data)
        });
    }

    fn remove(self) ->T {
        // log!("REMOVE {:#?}", self.id);
        EL_VARS.with(|el_vars| {
            el_vars
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
        EL_VARS.with(|el_vars| {
            let cmp_var_map = el_vars.borrow();
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

    pub fn use_ref<U>(self, user: impl FnOnce(&T)) {
        EL_VARS.with(|el_vars| {
            let cmp_var_map = el_vars.borrow();
            let data = cmp_var_map.data(&self.id);
            user(data)
        })
    }
}

pub trait CloneElVar<T: Clone + 'static> {
    fn inner(&self) -> T;
}

impl<T: Clone + 'static> CloneElVar<T> for ElVar<T> {
    fn inner(&self) -> T {
        self.map(Clone::clone)
    }
}
