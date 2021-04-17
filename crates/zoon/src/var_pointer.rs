use crate::runtime::VARS;
use crate::var_map::Id;
use crate::var::Var;
use std::marker::PhantomData;

pub struct VarPointer<T: 'static> {
    pub id: Id,
    phantom_data: PhantomData<T>,
}

impl<T> Eq for VarPointer<T> {}
impl<T> PartialEq for VarPointer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> std::fmt::Debug for VarPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}

impl<T> Copy for VarPointer<T> {}
impl<T> Clone for VarPointer<T> {
    fn clone(&self) -> VarPointer<T> {
        VarPointer::<T> {
            id: self.id,
            phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> VarPointer<T>
where
    T: 'static,
{
    pub fn new(var: &Var<T>) -> VarPointer<T> {
        VarPointer {
            id: var.id,
            phantom_data: PhantomData::<T>,
        }
    }

    pub fn exists(&self) -> bool {
        VARS.with(|vars| {
            vars.borrow().contains_id(self.id)
        })
    }

    pub fn set(&self, data: T) {
        VARS.with(|vars| {
            vars
                .borrow_mut()
                .insert(self.id, data)
        });
        // __Relations::refresh_dependents(&__Block::SVarPointer(self.id));
    }

    pub fn remove(self) -> Option<T> {
        VARS.with(|vars| {
            vars
                .borrow_mut()
                .remove::<T>(self.id)
        })
    }

    pub(crate) fn remove_mut(&self) -> Option<T> {
        VARS.with(|vars| {
            vars
                .borrow_mut()
                .remove::<T>(self.id)
        })
    }

    pub fn update(&self, updater: impl FnOnce(T) -> T) {
        let data = self.remove_mut().expect("an var data with the given id");
        self.set(updater(data));
    }

    pub fn update_mut(&self, updater: impl FnOnce(&mut T)) {
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

    pub fn map_mut<U>(&self, mapper: impl FnOnce(&mut T) -> U) -> U {
        let mut data = self.remove_mut().expect("an var data with the given id");
        let output = mapper(&mut data);
        self.set(data);
        output
    }

    pub fn use_ref(&self, user: impl FnOnce(&T)) {
        VARS.with(|vars| {
            let var_map = vars.borrow();
            let data = var_map.data(self.id)
                .expect("an var data with the given id");
            user(data)
        })
    }
}

pub trait CloneVarPointer<T: Clone + 'static> {
    fn inner(&self) -> T;
}

impl<T: Clone + 'static> CloneVarPointer<T> for VarPointer<T> {
    fn inner(&self) -> T {
        self.map(Clone::clone)
    }
}
