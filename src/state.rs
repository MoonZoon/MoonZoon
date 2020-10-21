use std::ops::{Add, Div, Mul, Sub};
use std::marker::PhantomData;
use std::cell::RefCell;
use std::collections::HashMap;
use std::any::Any;

thread_local! {
    static STATES: RefCell<StateMap> = RefCell::new(StateMap::new());
}

#[topo::nested]
pub fn use_state<T: 'static, F: FnOnce() -> T>(creator: F) -> State<T> {
    use_state_current(creator)
}

pub fn use_state_current<T: 'static, F: FnOnce() -> T>(creator: F) -> State<T> {
    let id = topo::CallId::current();

    let id_exists = STATES.with(|states| {
        states.borrow().contains_key(&id)
    });

    if !id_exists {
        let value = creator();
        STATES.with(|states| {
            states.borrow_mut().insert(id, value);
        });
    }

    State::new(id)
}

struct StateMap(HashMap<topo::CallId, Box<dyn Any>>);

impl StateMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn get<T: 'static>(&self, key: &topo::CallId) -> Option<&T> {
        self
            .0
            .get(key)?
            .downcast_ref::<Option<T>>()?
            .as_ref()
    }

    fn get_mut<T: 'static>(&mut self, key: &topo::CallId) -> Option<&mut T> {
        self
            .0
            .get_mut(key)?
            .downcast_mut::<Option<T>>()?
            .as_mut()
    }

    fn insert(&mut self, key: topo::CallId, value: impl Any) {
        self
            .0
            .insert(key, Box::new(Some(value)));
    }

    fn remove<T: 'static>(&mut self, key: &topo::CallId) -> Option<T> {
        self
            .0
            .remove(&key)?
            .downcast_mut::<Option<T>>()?
            .take()
    }

    fn contains_key(&self, key: &topo::CallId) -> bool {
        self
            .0
            .contains_key(&key)
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
    pub fn new(id: topo::CallId) -> State<T> {
        State {
            id,
            phantom_data: PhantomData,
        }
    }

    pub fn set(self, value: T) {
        STATES.with(|states| {
            states
                .borrow_mut()
                .insert(self.id, value)
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
        let mut value = self.remove().expect("a state value with the given key");
        updater(&mut value);
        self.set(value);
    }

    // pub fn state_exists(self) -> bool {
    //     state_exists_for_topo_id::<T>(self.id)
    // }

    pub fn get_with<F: FnOnce(&T) -> U, U>(self, getter: F) -> U {
        STATES.with(|states| {
            let state_map = states.borrow();
            let value = state_map.get(&self.id)
                .expect("a state value with the given key");
            getter(value)
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
