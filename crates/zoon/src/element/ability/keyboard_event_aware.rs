use crate::*;
use std::borrow::Borrow;

// ------ KeyboardEventAware ------

pub trait KeyboardEventAware<T: RawEl>: UpdateRawEl<T> + Sized {
    fn on_key_down_event(self, handler: impl FnOnce(KeyboardEvent) + Clone + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |event: events::KeyDown| {
                let keyboard_event = KeyboardEvent {
                    key: Key::from(event.key()),
                    raw_event: RawKeyboardEvent::KeyDown(event),
                };
                (handler.clone())(keyboard_event)
            })
        })
    }
}

// ------ KeyboardEvent ------

pub struct KeyboardEvent {
    key: Key,
    pub raw_event: RawKeyboardEvent,
}

impl KeyboardEvent {
    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn if_key(&self, key: impl Borrow<Key>, f: impl FnOnce()) {
        if &self.key == key.borrow() {
            f()
        }
    }
}

// ------ RawKeyboardEvent ------

pub enum RawKeyboardEvent {
    KeyDown(events::KeyDown)
}

// ------ Key ------

#[derive(PartialEq, Eq)]
pub enum Key {
    Enter,
    Escape,
    Other(String),
}

impl From<String> for Key {
    fn from(key: String) -> Self {
        match key.as_ref() {
            "Enter" => Key::Enter,
            "Escape" => Key::Escape,
            _ => Key::Other(key),
        }
    }
}
