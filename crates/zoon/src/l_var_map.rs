use std::collections::HashMap;
use std::any::Any;
use crate::tracked_call::TrackedCallId;

pub(crate) struct LVarMap {
    l_vars: HashMap<TrackedCallId, LVarMapValue>,
}

struct LVarMapValue {
    data: Box<dyn Any>,
}

impl LVarMap {
    pub(crate) fn new() -> Self {
        Self {
            l_vars: HashMap::new(),
        }
    }

    pub(crate) fn data<T: 'static>(&self, id: &TrackedCallId) -> &T {
        self
            .l_vars
            .get(id)
            .expect(&format!("the l_var with the id {:#?}", id))
            .data
            .downcast_ref::<Option<T>>()
            .expect(&format!("cast the l_var data with the id {:#?} to the required type", id))
            .as_ref()
            .expect(&format!("the l_var data with the id {:#?}", id))
    }

    pub(crate) fn insert(&mut self, id: TrackedCallId, data: impl Any) {
        self
            .l_vars
            .insert(id, LVarMapValue { 
                data: Box::new(Some(data)), 
            });
    }

    pub(crate) fn remove<T: 'static>(&mut self, id: &TrackedCallId) -> T {
        self
            .l_vars
            .remove(&id)
            .expect(&format!("remove the l_var data with the id {:#?}", id))
            .data
            .downcast_mut::<Option<T>>()
            .expect(&format!("cast the l_var data with the id {:#?} to the required type", id))
            .take()
            .expect(&format!("removed l_var data with the id {:#?}", id))
    }

    pub(crate) fn contains_id(&self, id: &TrackedCallId) -> bool {
        self
            .l_vars
            .contains_key(&id)
    }

    // pub(crate) fn update_revision(&mut self, id: &TrackedCallId) {
    //     let revision = self.revision;
    //     self
    //         .l_vars
    //         .get_mut(&id)
    //         .map(|l_var_map_value| {
    //             l_var_map_value.revision = revision
    //         }); 
    // }

    // pub(crate) fn remove_unused_and_toggle_revision(&mut self) -> Vec<Box<dyn Any>> {
    //     let current_revision = self.revision;
        
    //     let mut unused_data = Vec::new();
    //     let mut used_l_vars = HashMap::new();
        
    //     // @TODO: refactor once `HashMap::drain_filter` is stable (https://github.com/rust-lang/rust/issues/59618)
    //     for (id, value) in self.l_vars.drain() {
    //         if value.revision == current_revision {
    //             used_l_vars.insert(id, value);
    //         } else {
    //             unused_data.push(value.data);
    //         }
    //     }
    //     self.l_vars = used_l_vars;

    //     self.revision = !current_revision;
    //     unused_data
    // }
}
