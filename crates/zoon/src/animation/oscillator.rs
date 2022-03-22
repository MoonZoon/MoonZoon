use crate::*;

// ------ Oscillator ------

#[derive(Clone)]
pub struct Oscillator {
    value: Mutable<f64>,
}

impl Oscillator {
    pub fn new<T>(timeline: &Timeline<T>, keyframes: impl Fn(T) -> f64) -> Self {
        Self {
            value: Mutable::new(0.),
        }
    }

    pub fn signal(&self) -> impl Signal<Item = f64> {
        self.value.signal()
    }
}
