use crate::*;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

// ------ Timeline Data ------

struct Data<T> {
    queue: Arc<Mutex<VecDeque<Step<T>>>>,
    previous: Mutable<Step<T>>,
    next: Mutable<Option<Step<T>>>,
}

impl<T> Data<T> {
    fn new(state: T) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            previous: Mutable::new(Step::new(Duration::zero(), state)),
            next: Mutable::new(None),
        }
    }

    fn advance(&self, mut duration: Duration) {
        let next = self.next.take();
        if let Some(mut next) = next {
            if next.elapsed + duration > next.duration {
                duration = duration - (next.duration - next.elapsed);
                next.elapsed = next.duration;
                self.previous.set(next);
                self.advance(duration);
            } else {
                next.elapsed = next.elapsed + duration;
                self.next.set(Some(next))
            }
        } else {
            let next = self.queue.lock().unwrap_throw().pop_front();
            if let Some(next) = next {
                self.next.set(Some(next));
                self.advance(duration);
            }
        }
    }
}

impl<T> Clone for Data<T> {
    fn clone(&self) -> Self {
        Self {
            queue: Arc::clone(&self.queue),
            previous: self.previous.clone(),
            next: self.next.clone(),
        }
    }
}

// ------ Timeline ------

pub struct Timeline<T> {
    data: Data<T>,
    animation_frame: Option<AnimationFrame>,
}

impl<T> Clone for Timeline<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            animation_frame: self.animation_frame.clone(),
        }
    }
}

impl<T: 'static> Timeline<T> {
    pub fn new(state: T) -> Self {
        let data = Data::new(state);
        Self {
            data: data.clone(),
            animation_frame: Some(AnimationFrame::new(move |difference| {
                data.advance(difference);
            })),
        }
    }

    pub fn new_manual(state: T) -> Self {
        Self {
            data: Data::new(state),
            animation_frame: None,
        }
    }

    pub fn previous_signal_ref<U>(&self, mut mapper: impl FnMut(&T) -> U) -> impl Signal<Item = U> {
        self.data
            .previous
            .signal_ref(move |step| mapper(&step.state))
    }

    pub fn next_signal_ref<U>(
        &self,
        mut mapper: impl FnMut(Option<&T>) -> U,
    ) -> impl Signal<Item = U> {
        self.data
            .next
            .signal_ref(move |step| mapper(step.as_ref().map(|step| step.state.as_ref())))
    }

    pub fn previous_next_step_signals(&self) -> impl Signal<Item = (Step<T>, Option<Step<T>>)> {
        map_ref! {
            let previous = self.data.previous.signal_cloned(),
            let next = self.data.next.signal_cloned() => {
                (previous.clone(), next.clone())
            }
        }
    }

    pub fn push(&self, duration: Duration, state: T) {
        let step = Step::new(duration, state);
        self.data
            .queue
            .lock()
            .expect("failed to lock Timeline queue")
            .push_back(step);
    }

    pub fn advance(&self, duration: Duration) {
        self.data.advance(duration)
    }

    pub fn linear_animation(
        &self,
        mut keyframes: impl FnMut(&T) -> f64 + 'static,
    ) -> impl Signal<Item = f64> {
        self.previous_next_step_signals()
            .map(move |(previous, next)| {
                let previous_value = keyframes(previous.state.as_ref());
                if let Some(next) = next {
                    let next_value = keyframes(next.state.as_ref());
                    let progress = (next.elapsed.num_milliseconds() as f64)
                        / (next.duration.num_milliseconds() as f64);
                    previous_value + ((next_value - previous_value) * progress)
                } else {
                    previous_value
                }
            })
    }
}

// ------ Step ------

pub struct Step<T> {
    pub duration: Duration,
    pub state: Arc<T>,
    pub elapsed: Duration,
}

impl<T> Clone for Step<T> {
    fn clone(&self) -> Self {
        Self {
            duration: self.duration,
            state: Arc::clone(&self.state),
            elapsed: self.elapsed.clone(),
        }
    }
}

impl<T> Step<T> {
    fn new(duration: Duration, state: T) -> Self {
        Self {
            duration,
            state: Arc::new(state),
            elapsed: Duration::zero(),
        }
    }
}
