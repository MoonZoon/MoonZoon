use std::collections::HashMap;
use crate::tracked_call::__TrackedCallId;
use crate::tracked_call::TrackedCall;

pub(crate) struct TrackedCallMap {
    tracked_calls: HashMap<__TrackedCallId, TrackedCall>,
}


impl TrackedCallMap {
    pub(crate) fn new() -> Self {
        Self {
            tracked_calls: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, id: &__TrackedCallId) -> Option<&TrackedCall> {
        self
            .tracked_calls
            .get(id)
    }

    pub(crate) fn get_mut(&mut self, id: &__TrackedCallId) -> Option<&mut TrackedCall> {
        self
            .tracked_calls
            .get_mut(id)
    }

    pub(crate) fn insert(&mut self, id: __TrackedCallId, tracked_call: TrackedCall) {
        self
            .tracked_calls
            .insert(id, tracked_call);
    }

    // pub(crate) fn remove(&mut self, id: &__TrackedCallId) -> Option<TrackedCall> {
    //     self
    //         .tracked_calls
    //         .remove(&id)
    // }

    pub(crate) fn contains_id(&self, id: &__TrackedCallId) -> bool {
        self
            .tracked_calls
            .contains_key(&id)
    }

    pub(crate) fn reset_indices(&mut self, id: &__TrackedCallId) {
        for (selected_child_index, _) in self.tracked_calls.get_mut(id).unwrap().children.values_mut() {
            *selected_child_index = 0;
        }
    }
    
}
