use crate::*;

// ------ Oscillator ------

#[derive(Clone)]
pub struct Oscillator {
    value: Mutable<f64>,
}

impl Oscillator {
    pub fn new<T>(timeline: &Timeline<T>, keyframes: impl Fn(T) -> f64) -> Self {
        Self {
            value: Mutable::new(30.),
        }
    }

    pub fn signal(&self) -> impl Signal<Item = f64> {
        // @TODO remove dedupe?
        self.value.signal().dedupe()
    }
}
