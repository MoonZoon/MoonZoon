use crate::*;

// @TODO remove

// ------ Oscillator ------

#[derive(Clone)]
pub struct Oscillator {
    value: Mutable<f64>,
}

impl Oscillator {
    pub fn new<T: 'static>(timeline: &Timeline<T>, keyframes: impl Fn(T) -> f64) -> Self {
        // map_ref! {
        //     let arrived = timeline.arrived_step_signal_cloned(),
        //     let current = timeline.current_step_signal_cloned() => {

        //     }
        // };
        Self {
            value: Mutable::new(30.),
        }
    }

    pub fn signal(&self) -> impl Signal<Item = f64> {
        // @TODO remove dedupe?
        self.value.signal().dedupe()
    }
}
