use crate::runtime::STATES;
use crate::state::{State, CloneState};

#[topo::nested]
pub fn do_once<T>(f: impl FnOnce() -> T) -> Option<T> {
    let has_done = use_state(|| false);
    if has_done.get(){
        return None;
    }
    has_done.set(true);
    Some(f())
}

#[topo::nested]
pub fn use_state<T: 'static, F: FnOnce() -> T>(creator: F) -> State<T> {
    use_state_current(creator)
}

#[topo::nested]
pub fn new_state<T: 'static, F: FnOnce() -> T>(creator: F) -> State<T> {
    let count = use_state(|| 0);
    count.update(|count| *count += 1);
    topo::call_in_slot(&count.get(), || use_state_current(creator))
}

fn use_state_current<T: 'static, F: FnOnce() -> T>(creator: F) -> State<T> {
    let id = topo::CallId::current();

    let id_exists = STATES.with(|states| {
        states.borrow().contains_id(&id)
    });

    let data = if !id_exists {
        Some(creator())
    } else {
        None
    };

    STATES.with(|states| {
        let mut state_map = states.borrow_mut();
        if let Some(data) = data {
            state_map.insert(id, data);
        } else {
            state_map.update_revision(&id);
        }
    });

    State::new(id)
}
