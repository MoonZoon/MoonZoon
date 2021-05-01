use zoon::*;
use zoon::futures_signals::signal::{Signal, Mutable};
use std::rc::Rc;
use enclose::enc;

// ------ ------
//    Element 
// ------ ------

#[derive(Default)]
pub struct Counter {
    value: Option<i32>,
    value_signal: Option<Box<dyn Signal<Item = i32> + Unpin>>,
    on_change: Option<Rc<dyn Fn(CounterStep)>>,
    step: Option<CounterStep>,
}

pub type CounterStep = i32; 

impl Element for Counter {
    fn render(self) -> Dom {
        let on_change = self.on_change.map(|on_change| on_change);
        let step = self.step.unwrap_or(1);

        let value = self.value.unwrap_or_default();
        let state_value = self.value_signal.is_none().then(|| {
            Rc::new(Mutable::new(value))
        });

        let value_signal = self
            .value_signal
            .unwrap_or_else(|| Box::new(state_value.as_ref().unwrap_throw().signal()));

        let on_press_handler = move |delta: i32| {
            if let Some(state_value) = state_value {
                state_value.replace_with(|value| *value + delta);
            }
            if let Some(on_change) = on_change {
                on_change(delta);
            }
        };

        Row::new()
            .item(Button::new()
                .label("-")
                .on_press(enc!((on_press_handler) move || on_press_handler(-step)))
            )
            .item(El::new().child_signal(value_signal))
            .item(Button::new()
                .label("+")
                .on_press(move || on_press_handler(step))
            )
            .render()
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl Counter {
    pub fn value(mut self, value: i32) -> Self {
        self.value = Some(value);
        self
    }

    pub fn value_signal(mut self, value: impl Signal<Item  = i32> + Unpin + 'static) -> Self {
        self.value_signal = Some(Box::new(value));
        self
    }

    pub fn on_change(mut self, on_change: impl FnOnce(i32) + Clone + 'static) -> Self {
        self.on_change = Some(Rc::new(move |value| on_change.clone()(value)));
        self
    }

    pub fn step(mut self, step: i32) -> Self {
        self.step = Some(step);
        self
    }
}
