use zoon::{*, println};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;
use enclose::enc;
use zoon::dominator::{Dom, html};
use futures_signals::signal::{Signal, SignalExt, Mutable};
use once_cell::sync::OnceCell;
use std::collections::HashMap;

// ------ ------
//    Element 
// ------ ------

#[derive(Default)]
pub struct Counter {
    value: Option<i32>,
    value_signal: Option<Box<dyn Signal<Item = i32> + Unpin>>,
    on_change: Option<Rc<dyn Fn(i32)>>,
    step: Option<i32>,
}

impl Element for Counter {
    #[topo::nested]
    fn render(self) -> Dom {
        let on_change = self.on_change.map(|on_change| on_change);
        let step = self.step.unwrap_or(1);

        if let Some(value_signal) = self.value_signal {
            Row::new()
                .item({
                    let mut button = Button::new().label("-");
                    if let Some(on_change) = on_change.clone() {
                        button = button.on_press(move || on_change(-step));
                    }
                    button
                })
                .item(El::new()
                    .child_signal(value_signal)
                )
                .item({
                    let mut button = Button::new().label("+");
                    if let Some(on_change) = on_change {
                        button = button.on_press(move || on_change(step));
                    }
                    button
                })
                .render()
        } else {
            static __STATE_VALUES: OnceCell<RwLock<HashMap<CallId, Arc<Mutable<i32>>>>> = OnceCell::new();
            let __state_values = __STATE_VALUES.get_or_init(|| RwLock::new(HashMap::new()));
            let state_value = __state_values
                .write()
                .unwrap_throw()
                .entry(CallId::current())
                .or_default()
                .clone();

            if let Some(default_value) = self.value {
                state_value.set(default_value);
            }

            Row::new()
                .item(Button::new()
                    .label("-")
                    .on_press(enc!((state_value, on_change) move || {
                        state_value.replace_with(|value| *value - step);
                        if let Some(on_change) = on_change {
                            on_change(-step)
                        }
                    }))
                )
                .item(El::new()
                    .child_signal(state_value.signal())
                )
                .item(Button::new()
                    .label("+")
                    .on_press(enc!((state_value) move || {
                        state_value.replace_with(|value| *value + step);
                        if let Some(on_change) = on_change {
                            on_change(step)
                        }
                    }))
                )
                .render()
        }
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
