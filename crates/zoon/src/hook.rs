use crate::runtime::LVARS;
use crate::l_var::{LVar, CloneLVar};
use call_tree_macro::call_tree;
use crate::call_tree::{CallTree, CallId};

#[call_tree]
pub fn do_once<T>(f: impl FnOnce() -> T) -> Option<T> {
    let has_done = l_var(|| false);
    if has_done.inner(){
        return None;
    }
    has_done.set(true);
    Some(f())
}

#[call_tree]
pub fn l_var<T: 'static, F: FnOnce() -> T>(creator: F) -> LVar<T> {
    l_var_current(creator)
}

#[call_tree]
pub fn new_l_var<T: 'static, F: FnOnce() -> T>(creator: F) -> LVar<T> {
    let count = l_var(|| 0);
    count.update(|count| count + 1);
    CallTree::call_in_slot(&count.inner(), || l_var_current(creator))
}

fn l_var_current<T: 'static, F: FnOnce() -> T>(creator: F) -> LVar<T> {
    let id = CallId::current();

    let id_exists = LVARS.with(|l_vars| {
        l_vars.borrow().contains_id(&id)
    });

    let data = if id_exists {
        None
    } else {
        Some(creator())
    };

    LVARS.with(|l_vars| {
        let mut l_var_map = l_vars.borrow_mut();
        if let Some(data) = data {
            l_var_map.insert(id, data);
        } else {
            l_var_map.update_revision(&id);
        }
    });

    LVar::new(id)
}
