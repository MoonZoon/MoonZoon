use crate::*;
use std::borrow::Borrow;

// ------ Focusable ------

pub trait Focusable: UpdateRawEl<RawHtmlEl> + Sized {
    fn focus(self) -> Self {
        self.update_raw_el(|raw_el| raw_el.focus())
    } 
}

// ------ Styleable ------

pub trait Styleable<T: RawEl>: UpdateRawEl<T> + Sized {
    fn style<'a>(self, style: impl Style<'a>) -> Self {
        self.update_raw_el(|mut raw_el| {
            for (name, value) in style.into_css_props() {
                raw_el = raw_el.style(name, value);
            }
            raw_el
        })
    } 
}

// ------ KeyboardEventHandling ------

pub trait KeyboardEventHandling<T: RawEl>: UpdateRawEl<T> + Sized {
    fn on_key_down(self, handler: impl FnOnce(KeyboardEvent) + Clone + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |event: events::KeyDown| {
                let keyboard_event = KeyboardEvent {
                    key: Key::from(event.key())
                };
                (handler.clone())(keyboard_event)
            })
        })
    }
}

pub struct KeyboardEvent {
    key: Key
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

#[derive(PartialEq, Eq)]
pub enum Key {
    Enter,
    Other(String),
} 

impl From<String> for Key {
    fn from(event: String) -> Self {
        match event.as_str() {
            "Enter" => Key::Enter,
            _ => Key::Other(event)
        }
    }
}
