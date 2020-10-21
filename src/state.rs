use std::ops::{Add, Div, Mul, Sub};
use std::marker::PhantomData;
use std::cell::RefCell;
use std::collections::HashMap;
use std::any::Any;

thread_local! {
    static STATES: RefCell<StateMap> = RefCell::new(StateMap::new());
}

pub fn runtime_run_once(root: impl FnOnce()) {
    root();
    STATES.with(|states| {
        states
            .borrow_mut()
            .remove_unused_and_toggle_revision()
    });
}

#[topo::nested]
pub fn use_state<T: 'static, F: FnOnce() -> T>(creator: F) -> State<T> {
    use_state_current(creator)
}

pub fn use_state_current<T: 'static, F: FnOnce() -> T>(creator: F) -> State<T> {
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

struct StateMap {
    states: HashMap<topo::CallId, StateMapValue>,
    revision: bool,
}

struct StateMapValue {
    data: Box<dyn Any>,
    revision: bool,
}

impl StateMap {
    fn new() -> Self {
        Self {
            states: HashMap::new(),
            revision: false,
        }
    }

    fn data<T: 'static>(&self, id: &topo::CallId) -> Option<&T> {
        self
            .states
            .get(id)?
            .data
            .downcast_ref::<Option<T>>()?
            .as_ref()
    }

    fn insert(&mut self, id: topo::CallId, data: impl Any) {
        self
            .states
            .insert(id, StateMapValue { 
                data: Box::new(Some(data)), 
                revision: self.revision 
            });
    }

    fn remove<T: 'static>(&mut self, id: &topo::CallId) -> Option<T> {
        self
            .states
            .remove(&id)?
            .data
            .downcast_mut::<Option<T>>()?
            .take()
    }

    fn contains_id(&self, id: &topo::CallId) -> bool {
        self
            .states
            .contains_key(&id)
    }

    fn update_revision(&mut self, id: &topo::CallId) {
        let revision = self.revision;
        self
            .states
            .get_mut(&id)
            .map(|state_map_value| {
                state_map_value.revision = revision
            }); 
    }

    fn remove_unused_and_toggle_revision(&mut self) {
        let current_revision = self.revision;
        self
            .states
            .retain(|_, StateMapValue { revision, .. }| *revision == current_revision);

        self.revision = !current_revision;
    }
}


// #[derive(Debug)]
pub struct State<T> {
    pub id: topo::CallId,
    phantom_data: PhantomData<T>,
}

impl<T> std::fmt::Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}

impl<T> Copy for State<T> {}
impl<T> Clone for State<T> {
    fn clone(&self) -> State<T> {
        State::<T> {
            id: self.id,
            phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> State<T>
where
    T: 'static,
{
    fn new(id: topo::CallId) -> State<T> {
        State {
            id,
            phantom_data: PhantomData,
        }
    }

    pub fn set(self, data: T) {
        STATES.with(|states| {
            states
                .borrow_mut()
                .insert(self.id, data)
        });
    }

    pub fn remove(self) -> Option<T> {
        STATES.with(|states| {
            states
                .borrow_mut()
                .remove::<T>(&self.id)
        })
    }

    // pub fn reset_on_unmount(self) -> Self {
    //     on_unmount(move || self.delete());
    //     self
    // }

    pub fn update<F: FnOnce(&mut T) -> ()>(self, updater: F) {
        let mut data = self.remove().expect("a state data with the given id");
        updater(&mut data);
        self.set(data);
    }

    // pub fn state_exists(self) -> bool {
    //     state_exists_for_topo_id::<T>(self.id)
    // }

    pub fn get_with<F: FnOnce(&T) -> U, U>(self, getter: F) -> U {
        STATES.with(|states| {
            let state_map = states.borrow();
            let data = state_map.data(&self.id)
                .expect("a state data with the given id");
            getter(data)
        })
    }
}

pub trait CloneState<T: Clone + 'static> {
    fn get(&self) -> T;
}

impl<T: Clone + 'static> CloneState<T> for State<T> {
    fn get(&self) -> T {
        self.get_with(Clone::clone)
    }
}

// #[derive(Clone)]
// struct ChangedWrapper<T>(T);

// pub trait ChangedState {
//     fn changed(&self) -> bool;
// }

// impl<T> ChangedState for State<T>
// where
//     T: Clone + 'static + PartialEq,
// {
//     fn changed(&self) -> bool {
//         if let Some(old_state) = clone_state_with_topo_id::<ChangedWrapper<T>>(self.id) {
//             old_state.0 != self.get()
//         } else {
//             set_state_with_topo_id(ChangedWrapper(self.get()), self.id);
//             true
//         }
//     }
// }

impl<T> std::fmt::Display for State<T>
where
    T: std::fmt::Display + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_with(|t| format!("{}", t)))
    }
}

impl<T> Add for State<T>
where
    T: Copy + Add<Output = T> + 'static,
{
    type Output = T;

    fn add(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o + *s))
    }
}

impl<T> Mul for State<T>
where
    T: Copy + Mul<Output = T> + 'static,
{
    type Output = T;

    fn mul(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o * *s))
    }
}

impl<T> Div for State<T>
where
    T: Copy + Div<Output = T> + 'static,
{
    type Output = T;

    fn div(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o / *s))
    }
}

impl<T> Sub for State<T>
where
    T: Copy + Sub<Output = T> + 'static,
{
    type Output = T;

    fn sub(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o - *s))
    }
}
