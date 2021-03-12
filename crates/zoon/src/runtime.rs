use crate::state_map::StateMap;
use std::cell::RefCell;
use std::any::Any;

thread_local! {
    pub(crate) static STATES: RefCell<StateMap> = RefCell::new(StateMap::new());
}

pub fn run_once(root: impl FnOnce()) {
    root();
    let _unused_data: Vec<Box<dyn Any>> = STATES.with(|states| {
        states
            .borrow_mut()
            .remove_unused_and_toggle_revision()
    });
}
