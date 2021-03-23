use crate::runtime::{LVARS, CVARS};
use crate::l_var::{LVar, CloneLVar};
use crate::c_var::{CVar, CloneCVar};
use tracked_call_macro::tracked_call;
use crate::tracked_call::{TrackedCallId, __TrackedCall};
use crate::tracked_call_stack::__TrackedCallStack;
use crate::block_call_stack::__Block;
use crate::relations::__Relations;
use crate::log;

#[tracked_call]
pub fn do_once<T>(f: impl FnOnce() -> T) -> Option<T> {
    let has_done = l_var(|| false);
    if has_done.inner(){
        return None;
    }
    has_done.set(true);
    Some(f())
}

#[tracked_call]
pub fn l_var<T: 'static, F: FnOnce() -> T>(creator: F) -> LVar<T> {
    l_var_current(creator)
}

#[tracked_call]
pub fn c_var<T: 'static, F: FnOnce() -> T>(creator: F) -> CVar<T> {
    c_var_current(creator)
}

// #[tracked_call]
// pub fn new_l_var<T: 'static>(creator: impl FnOnce() -> T) -> LVar<T> {
//     let count = l_var(|| 0);
//     count.update(|count| count + 1);
//     CallTree::call_in_slot(&count.inner(), || l_var_current(creator))
// }

fn l_var_current<T: 'static>(creator: impl FnOnce() -> T) -> LVar<T> {
    let id = TrackedCallId::current();

    let l_var = LVar::new(id);

    let id_exists = LVARS.with(|l_vars| {
        l_vars.borrow().contains_id(&id)
    });

    if id_exists {
        return l_var;
    }

    // let block = __Block::LVar(id);
    // __Relations::add_dependency(block);

    let data = creator();

    LVARS.with(move |l_vars| {
        let mut l_var_map = l_vars.borrow_mut();
        l_var_map.insert(id, data);
        // else {
        //     l_var_map.update_revision(&id);
        // }
    });

    l_var
}

fn c_var_current<T: 'static>(creator: impl FnOnce() -> T) -> CVar<T> {
    let id = TrackedCallId::current();

    let c_var = CVar::new(id);

    let id_exists = CVARS.with(|c_vars| {
        c_vars.borrow().contains_id(&id)
    });

    if id_exists {
        return c_var;
    }

    // let block = __Block::LVar(id);
    // __Relations::add_dependency(block);

    let data = creator();

    CVARS.with(move |c_vars| {
        let mut c_var_map = c_vars.borrow_mut();
        c_var_map.insert(id, data);
        // else {
        //     c_var_map.update_revision(&id);
        // }
    });

    c_var
}
