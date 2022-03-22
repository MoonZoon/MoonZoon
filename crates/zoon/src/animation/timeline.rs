use crate::*;
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

// ------ Timeline ------

#[derive(Clone)]
pub struct Timeline<T> {
    queue: Arc<RwLock<VecDeque<Step<T>>>>,
    current: Mutable<Option<Step<T>>>,
    arrived: Mutable<Step<T>>,
    previous: Mutable<Option<Step<T>>>,
}

impl<T> Timeline<T> {
    pub fn new(state: T) -> Self {
        let step = Step::new(Duration::zero(), state);
        Self {
            queue: Arc::new(RwLock::new(VecDeque::new())),
            current: Mutable::new(Some(step.clone())),
            arrived: Mutable::new(step),
            previous: Mutable::new(None),
        }
    }

    pub fn arrived_signal(&self) -> impl Signal<Item = T>
    where
        T: Copy,
    {
        self.arrived.signal_cloned().map(|step| *step.state)
    }

    pub fn push(&self, duration: Duration, state: T) {
        let step = Step::new(duration, state);
        self.queue
            .write()
            .expect("failed to lock Timeline queue")
            .push_back(step);
    }
}

// ------ Step ------

struct Step<T> {
    duration: Duration,
    state: Arc<T>,
    elapsed: Arc<RwLock<Duration>>,
}

impl<T> Clone for Step<T> {
    fn clone(&self) -> Self {
        Self {
            duration: self.duration,
            state: Arc::clone(&self.state),
            elapsed: Arc::clone(&self.elapsed),
        }
    }
}

impl<T> Step<T> {
    fn new(duration: Duration, state: T) -> Self {
        Self {
            duration,
            state: Arc::new(state),
            elapsed: Arc::new(RwLock::new(Duration::zero())),
        }
    }
}
