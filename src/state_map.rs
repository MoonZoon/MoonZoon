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

    pub(crate) fn remove_unused_and_toggle_revision(&mut self) {
        let current_revision = self.revision;
        self
            .states
            .retain(|_, StateMapValue { revision, .. }| *revision == current_revision);

        self.revision = !current_revision;
    }
}
