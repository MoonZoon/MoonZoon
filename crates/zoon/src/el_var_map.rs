use griddle::HashMap;
use std::any::Any;
use crate::tracked_call::TrackedCallId;

pub(crate) struct ElVarMap {
    pub(crate) el_vars: HashMap<TrackedCallId, ElVarMapValue>,
}

pub(crate) struct ElVarMapValue {
    data: Box<dyn Any>,
}

impl ElVarMap {
    pub(crate) fn new() -> Self {
        Self {
            el_vars: HashMap::default(),
        }
    }

    pub(crate) fn data<T: 'static>(&self, id: &TrackedCallId) -> &T {
        self
            .el_vars
            .get(id)
            .unwrap_or_else(|| panic!("the el_var with the id {:#?}", id))
            .data
            .downcast_ref::<Option<T>>()
            .unwrap_or_else(|| panic!("cast the el_var data with the id {:#?} to the required type", id))
            .as_ref()
            .unwrap_or_else(|| panic!("the el_var data with the id {:#?}", id))
    }

    pub(crate) fn insert(&mut self, id: TrackedCallId, data: impl Any) {
        self
            .el_vars
            .insert(id, ElVarMapValue { 
                data: Box::new(Some(data)), 
            });
    }

    pub(crate) fn remove<T: 'static>(&mut self, id: &TrackedCallId) -> T {
        self
            .el_vars
            .remove(&id)
            .unwrap_or_else(|| panic!("remove the el_var data with the id {:#?}", id))
            .data
            .downcast_mut::<Option<T>>()
            .unwrap_or_else(|| panic!("cast the el_var data with the id {:#?} to the required type", id))
            .take()
            .unwrap_or_else(|| panic!("removed el_var data with the id {:#?}", id))
    }

    pub(crate) fn contains_id(&self, id: &TrackedCallId) -> bool {
        self
            .el_vars
            .contains_key(&id)
    }

    // pub(crate) fn update_revision(&mut self, id: &TrackedCallId) {
    //     let revision = self.revision;
    //     self
    //         .el_vars
    //         .get_mut(&id)
    //         .map(|el_var_map_value| {
    //             el_var_map_value.revision = revision
    //         }); 
    // }

    // pub(crate) fn remove_unused_and_toggle_revision(&mut self) -> Vec<Box<dyn Any>> {
    //     let current_revision = self.revision;
        
    //     let mut unused_data = Vec::new();
    //     let mut used_el_vars = HashMap::new();
        
    //     // @TODO: refactor once `HashMap::drain_filter` is stable (https://github.com/rust-lang/rust/issues/59618)
    //     for (id, value) in self.el_vars.drain() {
    //         if value.revision == current_revision {
    //             used_el_vars.insert(id, value);
    //         } else {
    //             unused_data.push(value.data);
    //         }
    //     }
    //     self.el_vars = used_el_vars;

    //     self.revision = !current_revision;
    //     unused_data
    // }
}
