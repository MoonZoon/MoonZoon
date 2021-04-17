use crate::runtime::VARS;
use crate::var_map::Id;
use std::marker::PhantomData;
use uuid::Uuid;

pub struct Var<T: 'static> {
    pub id: Id,
    phantom_data: PhantomData<T>,
}

impl<T> std::fmt::Debug for Var<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}

impl<T: 'static + Clone> Clone for Var<T> {
    fn clone(&self) -> Var<T> {
        Self::new(self.inner())
    }
}

impl<T: 'static> Drop for Var<T> {
    fn drop(&mut self) {
        self.remove_mut();
    }
}

impl<T> Var<T>
where
    T: 'static,
{
    pub fn new(value: T) -> Var<T> {
        let id = Uuid::new_v4().as_u128();

        VARS.with(move |vars| {
            vars.borrow_mut().insert(id, value);
        });

        Var {
            id,
            phantom_data: PhantomData,
        }
    }

    pub fn exists(&self) -> bool {
        VARS.with(|vars| {
            vars.borrow().contains_id(self.id)
        })
    }

    pub fn set(&mut self, data: T) {
        VARS.with(|vars| {
            vars
                .borrow_mut()
                .insert(self.id, data)
        });
        // __Relations::refresh_dependents(&__Block::SVar(self.id));
    }

    pub fn remove(self) -> Option<T> {
        VARS.with(|vars| {
            vars
                .borrow_mut()
                .remove::<T>(self.id)
        })
    }

    pub(crate) fn remove_mut(&mut self) -> Option<T> {
        VARS.with(|vars| {
            vars
                .borrow_mut()
                .remove::<T>(self.id)
        })
    }

    pub fn update(&mut self, updater: impl FnOnce(T) -> T) {
        let data = self.remove_mut().expect("an var data with the given id");
        self.set(updater(data));
    }

    pub fn update_mut(&mut self, updater: impl FnOnce(&mut T)) {
        let mut data = self.remove_mut().expect("an var data with the given id");
        updater(&mut data);
        self.set(data);
    }

    pub fn map<U>(&self, mapper: impl FnOnce(&T) -> U) -> U {
        VARS.with(|vars| {
            let var_map = vars.borrow();
            let data = var_map.data(self.id)
                .expect("an var data with the given id");
            mapper(data)
        })
    }

    pub fn map_mut<U>(&mut self, mapper: impl FnOnce(&mut T) -> U) -> U {
        let mut data = self.remove_mut().expect("an var data with the given id");
        let output = mapper(&mut data);
        self.set(data);
        output
    }

    pub fn use_ref<U>(&self, user: impl FnOnce(&T)) {
        VARS.with(|vars| {
            let var_map = vars.borrow();
            let data = var_map.data(self.id)
                .expect("an var data with the given id");
            user(data)
        })
    }
}

pub trait CloneVar<T: Clone + 'static> {
    fn inner(&self) -> T;
}

impl<T: Clone + 'static> CloneVar<T> for Var<T> {
    fn inner(&self) -> T {
        self.map(Clone::clone)
    }
}
