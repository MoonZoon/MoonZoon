use crate::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Tweened {
    oscillator: Oscillator,
    value: Mutable<f64>,
    value_updater: Arc<Mutex<Option<TaskHandle>>>,
    ease: SendWrapper<Arc<Mutex<dyn FnMut(f64) -> f64>>>,
}

impl Tweened {
    pub fn new(
        value: impl Into<f64>,
        transition_duration: Duration,
        ease: impl FnMut(f64) -> f64 + 'static,
    ) -> Self {
        Self {
            oscillator: Oscillator::new(transition_duration),
            value: Mutable::new(value.into()),
            value_updater: Arc::new(Mutex::new(None)),
            ease: SendWrapper::new(Arc::new(Mutex::new(ease))),
        }
    }

    pub fn new_and_signal(
        value: impl Into<f64>,
        transition_duration: Duration,
        ease: impl FnMut(f64) -> f64 + 'static,
    ) -> (Self, impl Signal<Item = f64>) {
        let this = Self::new(value, transition_duration, ease);
        let signal = this.signal();
        (this, signal)
    }

    pub fn signal(&self) -> impl Signal<Item = f64> {
        self.value.signal()
    }

    pub fn go_to(&self, target_value: impl Into<f64> + 'static) {
        let old_value = self.value.clone();
        let ease = self.ease.clone();
        self.oscillator.jump_to(false);
        *self.value_updater.lock().unwrap_throw() = Some(Task::start_droppable(
            self.oscillator
                .signal()
                .map(move |value| ease.lock().unwrap_throw()(value))
                .map(interpolate::linear(old_value.get(), target_value))
                .for_each_sync(move |value| old_value.set_neq(value)),
        ));
        self.oscillator.go_to(true);
    }

    pub fn get(&self) -> f64 {
        self.value.get()
    }
}
