use std::collections::HashMap;
use std::any::Any;

pub(crate) struct StateMap {
    states: HashMap<topo::CallId, StateMapValue>,
    revision: bool,
}

struct StateMapValue {
    data: Box<dyn Any>,
    revision: bool,
}

impl StateMap {
    pub(crate) fn new() -> Self {
        Self {
            states: HashMap::new(),
            revision: false,
        }
    }

    pub(crate) fn data<T: 'static>(&self, id: &topo::CallId) -> Option<&T> {
        self
            .states
            .get(id)?
            .data
            .downcast_ref::<Option<T>>()?
            .as_ref()
    }

    pub(crate) fn insert(&mut self, id: topo::CallId, data: impl Any) {
        self
            .states
            .insert(id, StateMapValue { 
                data: Box::new(Some(data)), 
                revision: self.revision 
            });
    }

    pub(crate) fn remove<T: 'static>(&mut self, id: &topo::CallId) -> Option<T> {
        self
            .states
            .remove(&id)?
            .data
            .downcast_mut::<Option<T>>()?
            .take()
    }

    pub(crate) fn contains_id(&self, id: &topo::CallId) -> bool {
        self
            .states
            .contains_key(&id)
    }

    pub(crate) fn update_revision(&mut self, id: &topo::CallId) {
        let revision = self.revision;
        self
            .states
            .get_mut(&id)
            .map(|state_map_value| {
                state_map_value.revision = revision
            }); 
    }

    pub(crate) fn remove_unused_and_toggle_revision(&mut self) -> Vec<Box<dyn Any>> {
        let current_revision = self.revision;
        
        let mut unused_data = Vec::new();
        let mut used_states = HashMap::new();
        
        // @TODO: refactor once `HashMap::drain_filter` is stable (https://github.com/rust-lang/rust/issues/59618)
        for (id, value) in self.states.drain() {
            if value.revision == current_revision {
                used_states.insert(id, value);
            } else {
                unused_data.push(value.data);
            }
        }
        self.states = used_states;

        self.revision = !current_revision;
        unused_data
    }
}
