use crate::runtime::SVARS;
use crate::s_var_map::Id;
use std::marker::PhantomData;

pub fn s_var<T: 'static, F: FnOnce() -> T>(id: Id, creator: F) -> SVar<T> {
    let id_exists = SVARS.with(|s_vars| {
        s_vars.borrow().contains_id(&id)
    });
    if !id_exists {
        let data = creator();
        SVARS.with(|s_vars| {
            s_vars.borrow_mut().insert(id, data);
        });
    }
    SVar::new(id)
}

pub struct SVar<T> {
    pub id: Id,
    phantom_data: PhantomData<T>,
}

impl<T> std::fmt::Debug for SVar<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}

impl<T> Copy for SVar<T> {}
impl<T> Clone for SVar<T> {
    fn clone(&self) -> SVar<T> {
        SVar::<T> {
            id: self.id,
            phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> SVar<T>
where
    T: 'static,
{
    pub(crate) fn new(id: Id) -> SVar<T> {
        SVar {
            id,
            phantom_data: PhantomData,
        }
    }

    pub fn set(self, data: T) {
        SVARS.with(|s_vars| {
            s_vars
                .borrow_mut()
                .insert(self.id, data)
        });
    }

    pub(crate) fn remove(self) -> Option<T> {
        SVARS.with(|s_vars| {
            s_vars
                .borrow_mut()
                .remove::<T>(&self.id)
        })
    }

    pub fn update(self, updater: impl FnOnce(T) -> T) {
        let data = self.remove().expect("an s_var data with the given id");
        self.set(updater(data));
    }

    pub fn update_mut(self, updater: impl FnOnce(&mut T)) {
        let mut data = self.remove().expect("an s_var data with the given id");
        updater(&mut data);
        self.set(data);
    }

    pub fn map<U>(self, mapper: impl FnOnce(&T) -> U) -> U {
        SVARS.with(|s_vars| {
            let s_var_map = s_vars.borrow();
            let data = s_var_map.data(&self.id)
                .expect("an s_var data with the given id");
            mapper(data)
        })
    }

    pub fn map_mut<U>(self, mapper: impl FnOnce(&mut T) -> U) -> U {
        let mut data = self.remove().expect("an s_var data with the given id");
        let output = mapper(&mut data);
        self.set(data);
        output
    }

    pub fn use_ref<U>(self, user: impl FnOnce(&T)) {
        SVARS.with(|s_vars| {
            let s_var_map = s_vars.borrow();
            let data = s_var_map.data(&self.id)
                .expect("an s_var data with the given id");
            user(data)
        })
    }
}

pub trait CloneSVar<T: Clone + 'static> {
    fn inner(&self) -> T;
}

impl<T: Clone + 'static> CloneSVar<T> for SVar<T> {
    fn inner(&self) -> T {
        self.map(Clone::clone)
    }
}
