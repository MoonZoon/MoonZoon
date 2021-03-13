use crate::l_var_map::LVarMap;
use std::cell::RefCell;
use std::any::Any;

thread_local! {
    pub(crate) static LVARS: RefCell<LVarMap> = RefCell::new(LVarMap::new());
}

pub fn run_once(root: impl FnOnce()) {
    root();
    let _unused_data: Vec<Box<dyn Any>> = LVARS.with(|l_vars| {
        l_vars
            .borrow_mut()
            .remove_unused_and_toggle_revision()
    });
}
