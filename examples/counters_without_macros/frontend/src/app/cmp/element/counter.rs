use zoon::{*, println};
use std::rc::Rc;
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
    value_mutable: Option<&'static Mutable<i32>>,
    on_change: Option<Rc<dyn Fn(i32)>>,
    step: Option<i32>,
}

impl Element for Counter {
    #[topo::nested]
    fn render(self) -> Dom {
        let on_change = self.on_change.map(|on_change| on_change);
        let step = self.step.unwrap_or(1);

        if let Some(value_mutable) = self.value_mutable {
            Row::new()
                .item({
                    let mut button = Button::new().label("-");
                    if let Some(on_change) = on_change.clone() {
                        button = button.on_press(move || on_change(value_mutable.get() - step));
                    }
                    button
                })
                .item(El::new()
                    .child_signal(value_mutable.signal())
                )
                .item({
                    let mut button = Button::new().label("+");
                    if let Some(on_change) = on_change {
                        button = button.on_press(move || on_change(value_mutable.get() + step));
                    }
                    button
                })
                .render()
        } else {
            static __STATE_VALUES: OnceCell<RwLock<HashMap<CallId, Mutable<i32>>>> = OnceCell::new();
            let __state_values = __STATE_VALUES.get_or_init(|| RwLock::new(HashMap::new()));
            let state_value = __state_values.write().unwrap_throw().entry(CallId::current()).or_default().clone();

            if let Some(default_value) = self.value {
                state_value.set(default_value);
            }

            let update_value = enc!((state_value) move |delta: i32| {
                state_value.replace_with(|value| *value + delta);
                if let Some(on_change) = on_change.clone() {
                    on_change(state_value.get());
                }
            });

            Row::new()
                .item(Button::new()
                    .label("-")
                    .on_press(enc!((update_value) move || update_value(-step)))
                )
                .item(El::new()
                    .child_signal(state_value.signal())
                )
                .item(Button::new()
                    .label("+")
                    .on_press(move || update_value(step))
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

    pub fn value_mutable(mut self, value: &'static Mutable<i32>) -> Self {
        self.value_mutable = Some(value);
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
