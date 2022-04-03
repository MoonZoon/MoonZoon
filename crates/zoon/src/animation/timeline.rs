use crate::*;
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock, Mutex},
};

// @TODO interrupt (hover multiple times)
// @TODO replace Timer with Raf

// ------ Timeline ------

pub struct Timeline<T: 'static> {
    queue: Arc<RwLock<VecDeque<Step<T>>>>,
    current: Mutable<Option<Step<T>>>,
    arrived: Mutable<Step<T>>,
    previous: Mutable<Option<Step<T>>>,
    timer: Arc<Mutex<Option<Timer>>>,
}

impl<T> Clone for Timeline<T> {
    fn clone(&self) -> Self {
        Self {
            queue: Arc::clone(&self.queue),
            current: self.current.clone(),
            arrived: self.arrived.clone(),
            previous: self.previous.clone(),
            timer: Arc::clone(&self.timer),
        }
    }
}

impl<T> Timeline<T> {
    pub fn new(state: T) -> Self {
        let step = Step::new(Duration::zero(), state);
        let this = Self {
            // @TODO channel? how to resolve interruptions with channel?
            queue: Arc::new(RwLock::new(VecDeque::new())),
            // @TODO do not overlap current and arrived (current None from start?)
            current: Mutable::new(Some(step.clone())),
            arrived: Mutable::new(step),
            previous: Mutable::new(None),
            timer: Arc::new(Mutex::new(None))
        };
        let timeline = this.clone();
        this.timer.lock().expect_throw("failed to lock Timeline timer").replace(
            Timer::new(50, move || timeline.jump(Duration::milliseconds(50)))
        );
        this
    }

    // @TODO recursion -> loop + refactor
    pub fn jump(&self, jump: Duration) {
        if self.current.map(Option::is_some) {
            let mut current_lock = self.current.lock_mut();
            let current = current_lock.as_mut().unwrap_throw();
            
            let mut elapsed = current.elapsed.lock_mut();
            let elapsed_with_jump = *elapsed + jump;
            
            let add_to_next_step = if elapsed_with_jump <= current.duration {
                Duration::zero()
            } else {
                elapsed_with_jump - current.duration
            };
            *elapsed = elapsed_with_jump - add_to_next_step;

            if current.duration == *elapsed {
                self.previous.set(Some(self.arrived.get_cloned()));
                self.arrived.set(current.clone());
                if not(add_to_next_step.is_zero()) {
                    drop(elapsed);
                    if let Some(next_step) = self.queue.write().expect_throw("failed to lock Timeline queue").pop_front() {
                        *current = next_step;
                        drop(current_lock);
                        self.jump(add_to_next_step);
                    } else {
                        *current_lock = None;
                    }
                }
            }
        } else {
            if let Some(next_step) = self.queue.write().expect_throw("failed to lock Timeline queue").pop_front() {
                self.current.set(Some(next_step));
                self.jump(jump);
            }
        }
    }

    // @TODO cloned version
    pub fn arrived_signal(&self) -> impl Signal<Item = T>
    where
        T: Copy,
    {
        self.arrived.signal_cloned().map(|step| *step.state)
    }

    // @TODO cloned version
    pub fn current_signal(&self) -> impl Signal<Item = Option<T>>
    where
        T: Copy,
    {
        self.current.signal_cloned().map(|step| step.map(|step| *step.state))
    }

    pub fn push(&self, duration: Duration, state: T) {
        let step = Step::new(duration, state);
        self.queue
            .write()
            .expect("failed to lock Timeline queue")
            .push_back(step);
    }

    // @TODO rename oscillator and keyframes
    pub fn oscillator(&self, keyframes: impl Fn(&T) -> f64) -> impl Signal<Item = f64> {
        let current_and_elapsed = self.current.signal_cloned().switch(|current| {
            if let Some(current) = current {
                return current.elapsed.signal().map(move |elapsed| Some((current.clone(), elapsed))).boxed_local();
            }
            always(None).boxed_local()
        });

        map_ref! {
            let arrived = self.arrived.signal_cloned(),
            let current_and_elapsed = current_and_elapsed => move {
                // @TODO if `arrived` elapsed != duration, compute the value from previous and arrived
                let arrived = keyframes(&arrived.state);
                if let Some((current, elapsed)) = current_and_elapsed  {
                    let progress = if current.duration.is_zero() {
                        0.
                    } else {
                        elapsed.num_milliseconds() as f64 / current.duration.num_milliseconds() as f64
                    };
                    let current = keyframes(&current.state);
                    return arrived + ((current - arrived) * progress);
                }
                arrived
            }
        }
    }
}

// ------ Step ------

pub struct Step<T> {
    duration: Duration,
    state: Arc<T>,
    elapsed: Mutable<Duration>,
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
            elapsed: Mutable::new(Duration::zero()),
        }
    }
}
