use std::marker::PhantomData;
use crate::{Var, SVar, Cache, ElVar, CmpVar, var_pointer::VarPointer, __Block, __Relations};

pub trait ToVarRef<T> {
    fn to_var_ref(&self) -> VarRef<T>;
}

impl<T> ToVarRef<T> for SVar<T> {
    fn to_var_ref(&self) -> VarRef<T> {
        VarRef {
            variable: Variable::SVar(*self),
            phantom_data: PhantomData,
        }
    }
}

impl<T> ToVarRef<T> for Cache<T> {
    fn to_var_ref(&self) -> VarRef<T> {
        VarRef {
            variable: Variable::Cache(*self),
            phantom_data: PhantomData,
        }
    }
}

impl<T> ToVarRef<T> for ElVar<T> {
    fn to_var_ref(&self) -> VarRef<T> {
        VarRef {
            variable: Variable::ElVar(*self),
            phantom_data: PhantomData,
        }
    }
}

impl<T> ToVarRef<T> for CmpVar<T> {
    fn to_var_ref(&self) -> VarRef<T> {
        VarRef {
            variable: Variable::CmpVar(*self),
            phantom_data: PhantomData,
        }
    }
}

impl<T> ToVarRef<T> for Var<T> {
    fn to_var_ref(&self) -> VarRef<T> {
        VarRef {
            variable: Variable::Var(VarPointer::new(self)),
            phantom_data: PhantomData,
        }
    }
}

pub struct VarRef<T: 'static> {
    pub variable: Variable<T>,
    phantom_data: PhantomData<T>,
}

impl<T> VarRef<T> {
    pub fn add_dependency(&self) {
        let block = match &self.variable {
            Variable::SVar(s_var) => __Block::SVar(s_var.id),
            Variable::Cache(cache) => __Block::Cache(cache.id),
            Variable::ElVar(_) => return,
            Variable::CmpVar(cmp_var) => __Block::CmpVar(cmp_var.id),
            Variable::Var(var_pointer) => __Block::Var(var_pointer.id),
        };
        __Relations::add_dependency(block)
    }
}

impl<T> Eq for VarRef<T> {}
impl<T> PartialEq for VarRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.variable == other.variable
    }
}

pub enum Variable<T: 'static> {
    SVar(SVar<T>),
    Cache(Cache<T>),
    ElVar(ElVar<T>),
    CmpVar(CmpVar<T>),
    Var(VarPointer<T>),
}

impl<T> Eq for Variable<T> {}
impl<T> PartialEq for Variable<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Variable::SVar(this), Variable::SVar(other)) => this == other,
            (Variable::Cache(this), Variable::Cache(other)) => this == other,
            (Variable::ElVar(this), Variable::ElVar(other)) => this == other,
            (Variable::CmpVar(this), Variable::CmpVar(other)) => this == other,
            (Variable::Var(this), Variable::Var(other)) => this == other,
            _ => false
        }
    }
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

impl<T> VarRef<T>
where
    T: 'static,
{
    pub fn exists(&self) -> bool {
        match &self.variable {
            Variable::SVar(_) => true,
            Variable::Cache(_) =>  true,
            Variable::ElVar(el_var) =>  el_var.exists(),
            Variable::CmpVar(cmp_var) =>  cmp_var.exists(),
            Variable::Var(var_pointer) =>  var_pointer.exists(),
        }
    }

    pub fn set(&self, data: T) {
        match &self.variable {
            Variable::SVar(s_var) => s_var.set(data),
            // @TODO make it work
            Variable::Cache(_) => todo!(),
            Variable::ElVar(el_var) =>  el_var.set(data),
            Variable::CmpVar(cmp_var) =>  cmp_var.set(data),
            Variable::Var(var_pointer) =>  var_pointer.set(data),
        }
    }

    // fn remove(self) ->T {
    // }

    pub fn update(&self, updater: impl FnOnce(T) -> T) {
        match &self.variable {
            Variable::SVar(s_var) => s_var.update(updater),
            Variable::Cache(cache) => cache.update(updater),
            Variable::ElVar(el_var) =>  el_var.update(updater),
            Variable::CmpVar(cmp_var) =>  cmp_var.update(updater),
            Variable::Var(var_pointer) =>  var_pointer.update(updater),
        }
    }

    pub fn update_mut(&self, updater: impl FnOnce(&mut T)) {
        match &self.variable {
            Variable::SVar(s_var) => s_var.update_mut(updater),
            Variable::Cache(cache) => cache.update_mut(updater),
            Variable::ElVar(el_var) =>  el_var.update_mut(updater),
            Variable::CmpVar(cmp_var) =>  cmp_var.update_mut(updater),
            Variable::Var(var_pointer) =>  var_pointer.update_mut(updater),
        }
    }

    pub fn map<U>(&self, mapper: impl FnOnce(&T) -> U) -> U {
        match &self.variable {
            Variable::SVar(s_var) => s_var.map(mapper),
            Variable::Cache(cache) => cache.map(mapper),
            Variable::ElVar(el_var) =>  el_var.map(mapper),
            Variable::CmpVar(cmp_var) =>  cmp_var.map(mapper),
            Variable::Var(var_pointer) =>  var_pointer.map(mapper),
        }
    }

    pub fn map_mut<U>(&self, mapper: impl FnOnce(&mut T) -> U) -> U {
        match &self.variable {
            Variable::SVar(s_var) => s_var.map_mut(mapper),
            Variable::Cache(cache) => cache.map_mut(mapper),
            Variable::ElVar(el_var) =>  el_var.map_mut(mapper),
            Variable::CmpVar(cmp_var) =>  cmp_var.map_mut(mapper),
            Variable::Var(var_pointer) =>  var_pointer.map_mut(mapper),
        }
    }

    pub fn use_ref(&self, user: impl FnOnce(&T)) {
        match &self.variable {
            Variable::SVar(s_var) => s_var.use_ref(user),
            Variable::Cache(cache) => cache.use_ref(user),
            Variable::ElVar(el_var) =>  el_var.use_ref(user),
            Variable::CmpVar(cmp_var) =>  cmp_var.use_ref(user),
            Variable::Var(var_pointer) =>  var_pointer.use_ref(user),
        }
    }
}

pub trait CloneVarRef<T: Clone + 'static> {
    fn inner(&self) -> T;
}

impl<T: Clone + 'static> CloneVarRef<T> for VarRef<T> {
    fn inner(&self) -> T {
        self.map(Clone::clone)
    }
}
