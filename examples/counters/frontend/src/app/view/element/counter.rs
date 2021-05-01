use zoon::*;
use zoon::futures_signals::signal::{Signal, Mutable};
use std::rc::Rc;
use enclose::enc;

// ------ ------
//    Element 
// ------ ------

element_macro!(counter, Counter::default());

#[derive(Default)]
pub struct Counter {
    value: Option<i32>,
    value_signal: Option<ValueSignal>,
    on_change: Option<OnChange>,
    step: Option<Step>,
}

pub type CounterStep = i32; 

impl Element for Counter {
    fn render(self) -> Dom {
        let value = self.value.unwrap_or_default();
        let state_value = self.value_signal.is_none().then(|| {
            Rc::new(Mutable::new(value))
        });

        let value_signal = self
            .value_signal
            .map(|ValueSignal(signal)| signal)
            .unwrap_or_else(|| Box::new(state_value.as_ref().unwrap_throw().signal()));

        let on_change = self.on_change.map(|OnChange(on_change)| on_change);
        let on_press_handler = move |delta: i32| {
            if let Some(state_value) = state_value {
                state_value.replace_with(|value| *value + delta);
            }
            if let Some(on_change) = on_change {
                on_change(delta);
            }
        };

        let step = self.step.map_or(1, |Step(step)| step);
        row![
            button![
                button::on_press(enc!((on_press_handler) move || on_press_handler(-step))),
                "-"
            ],
            el![
                el::child_signal(value_signal)
            ],
            button![
                button::on_press(move || on_press_handler(step)),
                "+"
            ]
        ].render()
    }
}

// ------ ------
//  Attributes 
// ------ ------

// ------ i32 ------

impl ApplyToElement<Counter> for i32 {
    fn apply_to_element(self, counter: &mut Counter) {
        counter.value = Some(self);
    }
}

// ------ counter::value_signal(...) -------

pub struct ValueSignal(Box<dyn Signal<Item = i32> + Unpin>);
pub fn value_signal(value: impl Signal<Item  = i32> + Unpin + 'static) -> ValueSignal {
    ValueSignal(Box::new(value))
}
impl ApplyToElement<Counter> for ValueSignal {
    fn apply_to_element(self, counter: &mut Counter) {
        counter.value_signal = Some(self);
    }
}

// ------ counter::on_change(...) -------

pub struct OnChange(Rc<dyn Fn(CounterStep)>);
pub fn on_change(on_change: impl FnOnce(CounterStep) + Clone + 'static) -> OnChange {
    OnChange(Rc::new(move |value| on_change.clone()(value)))
}
impl ApplyToElement<Counter> for OnChange {
    fn apply_to_element(self, counter: &mut Counter) {
        counter.on_change = Some(self);
    }
}

// ------ counter::step(...) -------

pub struct Step(CounterStep);
pub fn step(step: CounterStep) -> Step {
    Step(step)
}
impl ApplyToElement<Counter> for Step {
    fn apply_to_element(self, counter: &mut Counter) {
        counter.step = Some(self);
    }
}
