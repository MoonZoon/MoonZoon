use griddle::HashMap;
use std::any::Any;
use crate::tracked_call::TrackedCallId;

pub(crate) struct CmpVarMap {
    pub(crate) cmp_vars: HashMap<TrackedCallId, CmpVarMapValue>,
}

pub(crate) struct CmpVarMapValue {
    data: Box<dyn Any>,
}

impl CmpVarMap {
    pub(crate) fn new() -> Self {
        Self {
            cmp_vars: HashMap::with_capacity(10_000),
        }
    }

    pub(crate) fn data<T: 'static>(&self, id: &TrackedCallId) -> &T {
        self
            .cmp_vars
            .get(id)
            .unwrap_or_else(|| panic!("the cmp_var with the id {:#?}", id))
            .data
            .downcast_ref::<Option<T>>()
            .unwrap_or_else(|| panic!("cast the cmp_var data with the id {:#?} to the required type", id))
            .as_ref()
            .unwrap_or_else(|| panic!("the cmp_var data with the id {:#?}", id))
    }

    pub(crate) fn insert(&mut self, id: TrackedCallId, data: impl Any) {
        self
            .cmp_vars
            .insert(id, CmpVarMapValue { 
                data: Box::new(Some(data)), 
            });
    }

    pub(crate) fn remove<T: 'static>(&mut self, id: &TrackedCallId) -> T {
        self
            .cmp_vars
            .remove(&id)
            .unwrap_or_else(|| panic!("remove the cmp_var data with the id {:#?}", id))
            .data
            .downcast_mut::<Option<T>>()
            .unwrap_or_else(|| panic!("cast the cmp_var data with the id {:#?} to the required type", id))
            .take()
            .unwrap_or_else(|| panic!("removed cmp_var data with the id {:#?}", id))
    }

    pub(crate) fn contains_id(&self, id: &TrackedCallId) -> bool {
        self
            .cmp_vars
            .contains_key(&id)
    }

    // pub(crate) fn update_revision(&mut self, id: &TrackedCallId) {
    //     let revision = self.revision;
    //     self
    //         .cmp_vars
    //         .get_mut(&id)
    //         .map(|cmp_var_map_value| {
    //             cmp_var_map_value.revision = revision
    //         }); 
    // }

    // pub(crate) fn remove_unused_and_toggle_revision(&mut self) -> Vec<Box<dyn Any>> {
    //     let current_revision = self.revision;
        
    //     let mut unused_data = Vec::new();
    //     let mut used_cmp_vars = HashMap::new();
        
    //     // @TODO: refactor once `HashMap::drain_filter` is stable (https://github.com/rust-lang/rust/issues/59618)
    //     for (id, value) in self.cmp_vars.drain() {
    //         if value.revision == current_revision {
    //             used_cmp_vars.insert(id, value);
    //         } else {
    //             unused_data.push(value.data);
    //         }
    //     }
    //     self.cmp_vars = used_cmp_vars;

    //     self.revision = !current_revision;
    //     unused_data
    // }
}
