use std::marker::PhantomData;
use crate::{SVar, Cache, ElVar, CmpVar, var_pointer::VarPointer};

pub struct VarRef<T: 'static> {
    variable: Variable<T>,
    phantom_data: PhantomData<T>,
}

// #[derive(Debug)]
enum Variable<T: 'static> {
    SVar(SVar<T>),
    Cache(Cache<T>),
    ElVar(ElVar<T>),
    CmpVar(CmpVar<T>),
    Var(VarPointer<T>),
}

impl<T> Copy for Variable<T> {}
impl<T> Clone for Variable<T> {
    fn clone(&self) -> Variable<T> {
        match self {
            Variable::SVar(s_var) => Variable::SVar(s_var.clone()),
            Variable::Cache(cache) =>  Variable::Cache(cache.clone()),
            Variable::ElVar(el_var) =>  Variable::ElVar(el_var.clone()),
            Variable::CmpVar(cmp_var) =>  Variable::CmpVar(cmp_var.clone()),
            Variable::Var(var_pointer) =>  Variable::Var(var_pointer.clone()),
        }
    }
}

// impl<T> std::fmt::Debug for VarRef<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "({:#?})", self.variable)
//     }
// }

impl<T> Copy for VarRef<T> {}
impl<T> Clone for VarRef<T> {
    fn clone(&self) -> VarRef<T> {
        VarRef::<T> {
            variable: self.variable,
            phantom_data: PhantomData::<T>,
        }
    }
}

// impl<T> VarRef<T>
// where
//     T: 'static,
// {
//     pub(crate) fn new(id: TrackedCallId) -> VarRef<T> {
//         VarRef {
//             id,
//             phantom_data: PhantomData,
//         }
//     }

//     pub fn exists(self) -> bool {
//         EL_VARS.with(|el_vars| {
//             el_vars.borrow().contains_id(&self.id)
//         })
//     }

//     pub fn set(self, data: T) {
//         EL_VARS.with(|el_vars| {
//             el_vars
//                 .borrow_mut()
//                 .insert(self.id, data)
//         });
//     }

//     fn remove(self) ->T {
//         // log!("REMOVE {:#?}", self.id);
//         EL_VARS.with(|el_vars| {
//             el_vars
//                 .borrow_mut()
//                 .remove::<T>(&self.id)
//         })
//     }

//     pub fn update(self, updater: impl FnOnce(T) -> T) {
//         let data = self.remove();
//         self.set(updater(data));
//     }

//     pub fn update_mut(self, updater: impl FnOnce(&mut T)) {
//         let mut data = self.remove();
//         updater(&mut data);
//         self.set(data);
//     }

//     pub fn map<U>(self, mapper: impl FnOnce(&T) -> U) -> U {
//         EL_VARS.with(|el_vars| {
//             let cmp_var_map = el_vars.borrow();
//             let data = cmp_var_map.data(&self.id);
//             mapper(data)
//         })
//     }

//     pub fn map_mut<U>(self, mapper: impl FnOnce(&mut T) -> U) -> U {
//         let mut data = self.remove();
//         let output = mapper(&mut data);
//         self.set(data);
//         output
//     }

//     pub fn use_ref<U>(self, user: impl FnOnce(&T)) {
//         EL_VARS.with(|el_vars| {
//             let cmp_var_map = el_vars.borrow();
//             let data = cmp_var_map.data(&self.id);
//             user(data)
//         })
//     }
// }

// pub trait CloneVarRef<T: Clone + 'static> {
//     fn inner(&self) -> T;
// }

// impl<T: Clone + 'static> CloneVarRef<T> for VarRef<T> {
//     fn inner(&self) -> T {
//         self.map(Clone::clone)
//     }
// }
