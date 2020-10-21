use crate::state_map::StateMap;
use std::cell::RefCell;

thread_local! {
    pub(crate) static STATES: RefCell<StateMap> = RefCell::new(StateMap::new());
}

pub fn run_once(root: impl FnOnce()) {
    root();
    STATES.with(|states| {
        states
            .borrow_mut()
            .remove_unused_and_toggle_revision()
    });
}
