use crate::*;
use std::sync::{Arc, Mutex};

// ------ Oscillator Data ------

#[derive(Clone)]
struct Data {
    transition_duration: Arc<Mutex<Duration>>,
    value: Mutable<f64>,
    status: Arc<Mutex<Status>>,
    animation_loop: Arc<Mutex<Option<AnimationLoop>>>,
}

#[derive(PartialEq, Clone, Copy)]
enum Status {
    Stopped,
    GoingTo(f64),
    CycleWrap,
    Cycle(f64),
}

impl Data {
    fn new(transition_duration: Duration) -> Self {
        Self {
            transition_duration: Arc::new(Mutex::new(transition_duration)),
            value: Mutable::new(0.),
            status: Arc::new(Mutex::new(Status::Stopped)),
            animation_loop: Arc::new(Mutex::new(None)),
        }
    }

    fn start(&self, status: Status) {
        *self.status.lock().unwrap_throw() = status;
        let mut animation_loop = self.animation_loop.lock().unwrap_throw();
        if animation_loop.is_none() {
            let data = self.clone();
            *animation_loop = Some(AnimationLoop::new(move |difference| {
                data.advance(difference);
            }));
        }
    }

    fn stop(&self) {
        let mut animation_loop = self.animation_loop.lock().unwrap_throw();
        if animation_loop.is_some() {
            *animation_loop = None;
            *self.status.lock().unwrap_throw() = Status::Stopped;
        }
    }

    fn jump_to(&self, unit_interval_value: f64) {
        self.value.set_neq(unit_interval_value);
    }

    fn advance(&self, duration: Duration) {
        let status = *self.status.lock().unwrap_throw();
        let transition_duration = *self.transition_duration.lock().unwrap_throw();
        let value = self.value.get();

        match status {
            Status::Stopped => (),
            Status::GoingTo(target) => {
                let range = (duration.num_milliseconds() as f64)
                    / (transition_duration.num_milliseconds() as f64);
                let target_value_diff = target - value;
                let distance = target_value_diff.abs();
                if range >= distance {
                    self.stop();
                    self.jump_to(target);
                } else {
                    self.jump_to(value + range.copysign(target_value_diff))
                }
            }
            Status::CycleWrap => {
                let range = (duration.num_milliseconds() as f64)
                    / (transition_duration.num_milliseconds() as f64);
                self.jump_to((range + value).fract())
            }
            Status::Cycle(target) => {
                let range = (duration.num_milliseconds() as f64)
                    / (transition_duration.num_milliseconds() as f64);
                let target_value_diff = target - value;
                let distance = target_value_diff.abs();
                if range >= distance {
                    self.jump_to(target);
                    self.start(Status::Cycle(if target == 0. {
                        1.
                    } else if target == 1. {
                        0.
                    } else {
                        unreachable!()
                    }));
                } else {
                    self.jump_to(value + range.copysign(target_value_diff))
                }
            }
        }
    }
}

// ------ Oscillator ------

#[derive(Clone)]
pub struct Oscillator {
    data: Data,
}

impl Oscillator {
    pub fn new(transition_duration: Duration) -> Self {
        Self {
            data: Data::new(transition_duration),
        }
    }

    pub fn signal(&self) -> impl Signal<Item = f64> {
        self.data.value.signal()
    }

    pub fn cycle_wrap(&self) {
        self.data.start(Status::CycleWrap);
    }

    pub fn cycle(&self) {
        self.data.start(Status::Cycle(1.));
    }

    pub fn stop(&self) {
        self.data.stop();
    }

    pub fn go_to(&self, unit_interval_value: impl IntoF64) {
        let unit_interval_value = unit_interval_value.into_f64().clamp(0., 1.);
        self.data.start(Status::GoingTo(unit_interval_value));
    }

    pub fn jump_to(&self, unit_interval_value: impl IntoF64) {
        let unit_interval_value = unit_interval_value.into_f64().clamp(0., 1.);
        self.data.jump_to(unit_interval_value);
    }
}
