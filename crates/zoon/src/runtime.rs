use crate::l_var_map::LVarMap;
use crate::s_var_map::SVarMap;
use crate::root;
use crate::element::Element;
use std::cell::RefCell;
use std::any::Any;

thread_local! {
    pub(crate) static SVARS: RefCell<SVarMap> = RefCell::new(SVarMap::new());
}

thread_local! {
    pub(crate) static LVARS: RefCell<LVarMap> = RefCell::new(LVarMap::new());
}

thread_local! {
    pub(crate) static ROOT_CMP: RefCell<Option<Box<dyn Fn() -> Box<dyn Element>>>> = RefCell::new(None);
}

pub fn rerender() {
    root();
    let _unused_data: Vec<Box<dyn Any>> = LVARS.with(|l_vars| {
        l_vars
            .borrow_mut()
            .remove_unused_and_toggle_revision()
    });
}
