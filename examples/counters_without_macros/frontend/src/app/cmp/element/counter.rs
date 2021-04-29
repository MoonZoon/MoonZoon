use zoon::{*, println};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;
use enclose::enc;
use zoon::dominator::{Dom, html};
use futures_signals::signal::{Signal, SignalExt, Mutable};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use zoon::ahash;
use std::hash::{Hash, Hasher};

// ------ ------
//    Element 
// ------ ------

pub struct Counter {
    key: u64,
    after_removes: Vec<Box<dyn FnOnce()>>,
    value: Option<i32>,
    value_signal: Option<Box<dyn Signal<Item = i32> + Unpin>>,
    on_change: Option<Rc<dyn Fn(i32)>>,
    step: Option<i32>,
}

impl Counter {
    #[topo::nested]
    pub fn new() -> Self {
        Self::new_with_key(CallId::current())
    }

    #[topo::nested]
    pub fn new_with_key(key: impl Hash) -> Self {
        // @TODO

        // let parent_call_id = CallId::parent();
        let parent_call_id = ();

        let mut hasher = ahash::AHasher::default();
        (parent_call_id, key).hash(&mut hasher);
        let key = hasher.finish();

        Self {
            key,
            after_removes: Vec::new(),
            value: None,
            value_signal: None,
            on_change: None,
            step: None,
        }
    } 
}

impl Element for Counter {
    fn render(mut self) -> Dom {
        let on_change = self.on_change.map(|on_change| on_change);
        let step = self.step.unwrap_or(1);

        let mut row = if let Some(value_signal) = self.value_signal {
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
        } else {
            let key = self.key;
            static __STATE_VALUES: OnceCell<RwLock<IntMap<u64, Arc<Mutable<i32>>>>> = OnceCell::new();
            let __state_values = __STATE_VALUES.get_or_init(|| RwLock::new(IntMap::default()));
            let state_value = __state_values
                .write()
                .unwrap_throw()
                .entry(key)
                .or_default()
                .clone();

            self.after_removes.push(Box::new(move || {
                __state_values
                    .write()
                    .unwrap_throw()
                    .remove(&key);
                println!("Counter removed!");
            }));

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
        };

        row = row.after_removes(self.after_removes);
        row.render()
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl Counter {
    pub fn after_remove(mut self, after_remove: impl FnOnce() + 'static) -> Self {
        self.after_removes.push(Box::new(after_remove));
        self
    }

    pub fn after_removes(mut self, after_removes: Vec<Box<dyn FnOnce()>>) -> Self {
        self.after_removes.extend(after_removes);
        self
    }

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
