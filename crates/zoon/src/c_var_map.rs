use griddle::HashMap;
use std::any::Any;
use crate::tracked_call::TrackedCallId;

pub struct CVarMap {
    pub(crate) c_vars: HashMap<TrackedCallId, CVarMapValue>,
}

pub(crate) struct CVarMapValue {
    data: Box<dyn Any>,
}

impl CVarMap {
    pub(crate) fn new() -> Self {
        Self {
            c_vars: HashMap::with_capacity(10_000),
        }
    }

    pub(crate) fn data<T: 'static>(&self, id: &TrackedCallId) -> &T {
        self
            .c_vars
            .get(id)
            .unwrap_or_else(|| panic!("the c_var with the id {:#?}", id))
            .data
            .downcast_ref::<Option<T>>()
            .unwrap_or_else(|| panic!("cast the c_var data with the id {:#?} to the required type", id))
            .as_ref()
            .unwrap_or_else(|| panic!("the c_var data with the id {:#?}", id))
    }

    pub fn insert(&mut self, id: TrackedCallId, data: impl Any) {
        self
            .c_vars
            .insert(id, CVarMapValue { 
                data: Box::new(Some(data)), 
            });
    }

    pub fn remove<T: 'static>(&mut self, id: &TrackedCallId) -> T {
        self
            .c_vars
            .remove(&id)
            .unwrap_or_else(|| panic!("remove the c_var data with the id {:#?}", id))
            .data
            .downcast_mut::<Option<T>>()
            .unwrap_or_else(|| panic!("cast the c_var data with the id {:#?} to the required type", id))
            .take()
            .unwrap_or_else(|| panic!("removed c_var data with the id {:#?}", id))
    }

    pub(crate) fn contains_id(&self, id: &TrackedCallId) -> bool {
        self
            .c_vars
            .contains_key(&id)
    }

    // pub(crate) fn update_revision(&mut self, id: &TrackedCallId) {
    //     let revision = self.revision;
    //     self
    //         .c_vars
    //         .get_mut(&id)
    //         .map(|c_var_map_value| {
    //             c_var_map_value.revision = revision
    //         }); 
    // }

    // pub(crate) fn remove_unused_and_toggle_revision(&mut self) -> Vec<Box<dyn Any>> {
    //     let current_revision = self.revision;
        
    //     let mut unused_data = Vec::new();
    //     let mut used_c_vars = HashMap::new();
        
    //     // @TODO: refactor once `HashMap::drain_filter` is stable (https://github.com/rust-lang/rust/issues/59618)
    //     for (id, value) in self.c_vars.drain() {
    //         if value.revision == current_revision {
    //             used_c_vars.insert(id, value);
    //         } else {
    //             unused_data.push(value.data);
    //         }
    //     }
    //     self.c_vars = used_c_vars;

    //     self.revision = !current_revision;
    //     unused_data
    // }
}
