use std::collections::HashMap;
use std::any::Any;

pub type Id = u128;

pub(crate) struct SVarMap {
    s_vars: HashMap<Id, SVarMapValue>,
}

struct SVarMapValue {
    data: Box<dyn Any>,
}

impl SVarMap {
    pub(crate) fn new() -> Self {
        Self {
            s_vars: HashMap::new(),
        }
    }

    pub(crate) fn data<T: 'static>(&self, id: Id) -> Option<&T> {
        self
            .s_vars
            .get(&id)?
            .data
            .downcast_ref::<Option<T>>()?
            .as_ref()
    }

    pub(crate) fn insert(&mut self, id: Id, data: impl Any) {
        self
            .s_vars
            .insert(id, SVarMapValue { 
                data: Box::new(Some(data)), 
            });
    }

    pub(crate) fn remove<T: 'static>(&mut self, id: Id) -> Option<T> {
        self
            .s_vars
            .remove(&id)?
            .data
            .downcast_mut::<Option<T>>()?
            .take()
    }

    pub(crate) fn contains_id(&self, id: Id) -> bool {
        self
            .s_vars
            .contains_key(&id)
    }
}
