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
    value_signal: Option<Box<dyn Signal<Item = i32>>>,
    on_change: Option<Rc<dyn Fn(i32)>>,
    step: Option<i32>,
}

impl Element for Counter {
    #[topo::nested]
    fn render(self) -> Dom {
        static __VALUES: OnceCell<RwLock<HashMap<CallId, Mutable<i32>>>> = OnceCell::new();
        let __values = __VALUES.get_or_init(|| RwLock::new(HashMap::new()));
        let value = __values.write().unwrap_throw().entry(CallId::current()).or_default().clone();

        let on_change = self.on_change.map(|on_change| on_change);
        let step = self.step.unwrap_or(1);
        
        if let Some(required_value) = self.value {
            value.set(required_value);
        }

        let update_value = enc!((value) move |delta: i32| {
            value.replace_with(|value| *value + delta);
            if let Some(on_change) = on_change.clone() {
                on_change(value.get());
            }
        });

        Row::new()
            .item(Button::new()
                .label("-")
                .on_press(enc!((update_value) move || update_value(-step)))
            )
            .item(El::new()
                .child_signal(value.signal())
            )
            .item(Button::new()
                .label("+")
                .on_press(move || update_value(step))
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

    pub fn value_signal(mut self, value: impl Signal<Item = i32> + 'static) -> Self {
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
