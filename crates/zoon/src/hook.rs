use crate::runtime::{EL_VARS, C_VARS, CMP_VARS};
use crate::el_var::{ElVar, CloneElVar};
use crate::cmp_var::{CmpVar, CloneCmpVar};
use crate::c_var::{CVar, CloneCVar};
use tracked_call_macro::tracked_call;
use crate::tracked_call::{TrackedCallId, __TrackedCall};
use crate::tracked_call_stack::__TrackedCallStack;
use crate::component_call_stack::__ComponentCallStack;
use crate::block_call_stack::__BlockCallStack;
use crate::block_call_stack::__Block;
use crate::component::{ComponentChild, __ComponentData};
use crate::relations::__Relations;
use crate::log;

#[tracked_call]
pub fn do_once<T>(f: impl FnOnce() -> T) -> Option<T> {
    let has_done = el_var(|| false);
    if has_done.inner(){
        return None;
    }
    has_done.set(true);
    Some(f())
}

#[tracked_call]
pub fn el_var<T: 'static, F: FnOnce() -> T>(creator: F) -> ElVar<T> {
    el_var_current(creator)
}

#[tracked_call]
pub fn cmp_var<T: 'static, F: FnOnce() -> T>(creator: F) -> CmpVar<T> {
    cmp_var_current(creator)
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

fn el_var_current<T: 'static>(creator: impl FnOnce() -> T) -> ElVar<T> {
    let id = TrackedCallId::current();

    let el_var = ElVar::new(id);

    let id_exists = EL_VARS.with(|el_vars| {
        el_vars.borrow().contains_id(&id)
    });

    if id_exists {
        return el_var;
    }

    let data = creator();

    EL_VARS.with(move |el_vars| {
        let mut el_var_map = el_vars.borrow_mut();
        el_var_map.insert(id, data);
    });

    if let Some(component_data_id) = __ComponentCallStack::last() {
        C_VARS.with(move |c_vars| {
            // log!("push ComponentChild::ElVar");
            let mut c_vars = c_vars.borrow_mut();
    
            let mut component_data = c_vars.remove::<__ComponentData>(&component_data_id);
            component_data.children.push(ComponentChild::ElVar(id));
    
            c_vars.insert(component_data_id, component_data);
        });
    }

    el_var
}

fn cmp_var_current<T: 'static>(creator: impl FnOnce() -> T) -> CmpVar<T> {
    let id = TrackedCallId::current();

    let cmp_var = CmpVar::new(id);

    let id_exists = CMP_VARS.with(|cmp_vars| {
        cmp_vars.borrow().contains_id(&id)
    });

    if id_exists {
        return cmp_var;
    }

    let block = __Block::CmpVar(id);
    __Relations::add_dependency(block);

    let data = creator();

    CMP_VARS.with(move |cmp_vars| {
        let mut cmp_var_map = cmp_vars.borrow_mut();
        cmp_var_map.insert(id, data);
    });

    if let Some(__Block::Cmp(component_data_id)) = __BlockCallStack::last() {
        C_VARS.with(move |c_vars| {
            // log!("push ComponentChild::CmpVar");
            let mut c_vars = c_vars.borrow_mut();
    
            let mut component_data = c_vars.remove::<__ComponentData>(&component_data_id);
            component_data.children.push(ComponentChild::CmpVar(id));
    
            c_vars.insert(component_data_id, component_data);
        });
    }

    cmp_var
}

fn c_var_current<T: 'static>(creator: impl FnOnce() -> T) -> CVar<T> {
    let id = TrackedCallId::current();

    let c_var = CVar::new(id);

    let id_exists = C_VARS.with(|c_vars| {
        c_vars.borrow().contains_id(&id)
    });

    if id_exists {
        return c_var;
    }

    let data = creator();

    C_VARS.with(move |c_vars| {
        let mut c_var_map = c_vars.borrow_mut();
        c_var_map.insert(id, data);
    });

    c_var
}
