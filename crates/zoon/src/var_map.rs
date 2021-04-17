use std::collections::HashMap;
use std::any::Any;

pub type Id = u128;

pub(crate) struct VarMap {
    vars: HashMap<Id, VarMapValue>,
}

struct VarMapValue {
    data: Box<dyn Any>,
}

impl VarMap {
    pub(crate) fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub(crate) fn data<T: 'static>(&self, id: Id) -> Option<&T> {
        self
            .vars
            .get(&id)?
            .data
            .downcast_ref::<Option<T>>()?
            .as_ref()
    }

    pub(crate) fn insert(&mut self, id: Id, data: impl Any) {
        self
            .vars
            .insert(id, VarMapValue { 
                data: Box::new(Some(data)), 
            });
    }

    pub(crate) fn remove<T: 'static>(&mut self, id: Id) -> Option<T> {
        self
            .vars
            .remove(&id)?
            .data
            .downcast_mut::<Option<T>>()?
            .take()
    }

    pub(crate) fn contains_id(&self, id: Id) -> bool {
        self
            .vars
            .contains_key(&id)
    }
}
