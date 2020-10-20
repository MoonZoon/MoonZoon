use crate::{
    store::{STORE, SlottedKey, StorageKey, TopoKey, Context},
    unmount::Unmount,
};
use std::{cell::RefCell, collections::HashSet};
use crate::state_access::{StateAccess, CloneState};

/// call the provided function once and once only
/// returns a unmmunt which will allow the do_once
/// to repeat if .execute_if_activated() is called on the unmount.
/// Example
///
/// do_once(||{
///     println!("This will print only once");
/// });
#[topo::nested]
pub fn do_once<F: FnMut() -> ()>(mut func: F) -> StateAccess<bool> {
    let has_done = use_state(|| false);
    if !has_done.get() {
        func();
        has_done.set(true);
    }
    has_done
}

///
/// Constructs a T accessor. T is stored keyed to the current topological
/// context. The accessor always references this context therefore can you can
/// set/update/ or get this T from anywhere.
///
///  The passed closure is only used for the first initialisation of state.
///  Subsequent evaluations of this function just returns the accessor.
///  Only one type per context can be stored in this way.
///
/// # Examples
///
/// ```
/// let my_string =  use_state(|| "foo".to_string());
/// ...
/// ...
///  // Maybe in a Callback...
/// my_string.set("bar")
/// ```
///
/// This stores a string "foo" in the current topological context,
/// which is later set to "bar", in some other part of the program.
///
/// You can store Clone or non-Clone types. Although non-Clone types need
/// to be read via their accessor in a more restrictive way.
// in a parent context.
#[topo::nested]
pub fn use_state<T: 'static, F: FnOnce() -> T>(data_fn: F) -> StateAccess<T> {
    use_state_current(data_fn)
}

///
///  Uses the current topological id to create a new state accessor
///
pub fn use_state_current<T: 'static, F: FnOnce() -> T>(data_fn: F) -> StateAccess<T> {
    let id = topo::CallId::current();
    let ctx = get_state_slotted_key_struct_if_in_context();

    let id = TopoKey { ctx, id };

    if !state_exists_for_topo_id::<T>(id) {
        set_state_with_topo_id::<T>(data_fn(), id);
    }
    // mark_id_as_active(current_id);
    StateAccess::new(id)
}

#[topo::nested]
pub fn new_state<T: 'static, F: FnOnce() -> T>(data_fn: F) -> StateAccess<T> {
    let count = use_state(|| 0);
    count.update(|c| *c += 1);
    topo::call_in_slot(&count.get(), || use_state_current(data_fn))
}

/// Sets the state of type T keyed to the given TopoId
pub fn set_state_with_topo_id<T: 'static>(data: T, current_id: TopoKey) {
    STORE.with(|store_refcell| {
        store_refcell
            .borrow_mut()
            .set_state_with_id::<T>(data, &StorageKey::TopoKey(current_id))
    });
}

pub fn state_exists_for_topo_id<T: 'static>(id: TopoKey) -> bool {
    STORE.with(|store_refcell| {
        store_refcell
            .borrow()
            .state_exists_with_id::<T>(StorageKey::TopoKey(id))
    })
}

fn get_state_slotted_key_struct_if_in_context() -> Option<SlottedKey> {
    illicit::get::<RefCell<Context>>()
        .ok()
        .map(|ctx| {
            if let StorageKey::SlottedKey(key) = ctx.borrow_mut().key {
                Some(key)
            } else {
                None
            }
        })
        .flatten()
}

/// Clones the state of type T keyed to the given TopoId
pub fn clone_state_with_topo_id<T: 'static + Clone>(id: TopoKey) -> Option<T> {
    STORE.with(|store_refcell| {
        store_refcell
            .borrow_mut()
            .get_state_with_id::<T>(&StorageKey::TopoKey(id))
            .cloned()
    })
}

pub fn remove_state_with_topo_id<T: 'static>(id: TopoKey) -> Option<T> {
    STORE.with(|store_refcell| {
        store_refcell
            .borrow_mut()
            .remove_state_with_id::<T>(&StorageKey::TopoKey(id))
    })
}

/// Provides mutable access to the stored state type T.
///
/// Example:
///
/// ```
/// update_state_with_topo_id::<Vec<String>>( topo::CallId::current(), |v|
///     v.push("foo".to_string()
/// )
pub fn update_state_with_topo_id<T: 'static, F: FnOnce(&mut T) -> ()>(id: TopoKey, func: F) {
    let mut item = remove_state_with_topo_id::<T>(id)
        .expect("You are trying to update a type state that doesnt exist in this context!");
    func(&mut item);
    set_state_with_topo_id(item, id);
}

pub fn read_state_with_topo_id<T: 'static, F: FnOnce(&T) -> R, R>(id: TopoKey, func: F) -> R {
    let item = remove_state_with_topo_id::<T>(id)
        .expect("You are trying to read a type state that doesnt exist in this context!");
    let read = func(&item);
    set_state_with_topo_id(item, id);
    read
}

/// Rudamentary Garbage Collection
/// purges all unseen ids' state
/// then resets the suneen ids list.

// pub fn purge_and_reset_unseen_ids() {
//     purge_unseen_ids();
//     reset_unseen_id_list();
// }

/// Rudamentary Garbage Collection
///
/// Copies all ids in the storage map to an unseen_id list.
/// Each Id is then removed if accessed
///
/// Paired with purge_unseen_ids to remove state for ids that have not been
/// accessed

pub fn reset_unseen_id_list() {
    STORE.with(|store_refcell| {
        let mut store_mut = store_refcell.borrow_mut();

        store_mut.unseen_ids = HashSet::new();
        let ids = store_mut
            .id_to_key_map
            .keys()
            .filter_map(|k| match k {
                StorageKey::SlottedKey(_) => None,
                StorageKey::TopoKey(key) => Some(key),
            })
            .cloned()
            .collect::<Vec<_>>();

        for id in ids {
            store_mut.unseen_ids.insert(id);
        }
    })
}

pub fn unseen_ids() -> Vec<TopoKey> {
    STORE.with(|store_refcell| {
        let store_mut = store_refcell.borrow_mut();
        store_mut.unseen_ids.iter().cloned().collect::<Vec<_>>()
    })
}

pub fn execute_and_remove_unmounts() {
    for id in unseen_ids() {
        if state_exists_for_topo_id::<Unmount>(id) {
            read_state_with_topo_id::<Unmount, _, _>(id, |dt| dt.execute_if_activated());
            remove_state_with_topo_id::<Unmount>(id);
        }
    }
}

#[topo::nested]
pub fn on_unmount<F: Fn() -> () + 'static>(unmount_fn: F) -> StateAccess<Unmount> {
    use_state(|| Unmount::new(unmount_fn))
}
