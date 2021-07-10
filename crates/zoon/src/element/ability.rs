use crate::*;
use std::borrow::Borrow;

// ------ Focusable ------

pub trait Focusable: UpdateRawEl<RawHtmlEl> + Sized {
    fn focus(self) -> Self {
        self.update_raw_el(|raw_el| raw_el.focus())
    }
}

// ------ Styleable ------

pub trait Styleable<'a, T: RawEl>: UpdateRawEl<T> + Sized {
    fn s(self, style: impl Style<'a>) -> Self {
        self.update_raw_el(|raw_el| style.update_raw_el_style(raw_el))
    }
}

// ------ KeyboardEventAware ------

pub trait KeyboardEventAware<T: RawEl>: UpdateRawEl<T> + Sized {
    fn on_key_down(self, handler: impl FnOnce(KeyboardEvent) + Clone + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |event: events::KeyDown| {
                let keyboard_event = KeyboardEvent {
                    key: Key::from(event.key()),
                };
                (handler.clone())(keyboard_event)
            })
        })
    }
}

pub struct KeyboardEvent {
    key: Key,
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
            _ => Key::Other(event),
        }
    }
}

// ------ Hoverable ------

pub trait Hoverable<T: RawEl>: UpdateRawEl<T> + Sized {
    fn on_hovered_change(self, handler: impl FnOnce(bool) + Clone + 'static) -> Self {
        let mouse_enter_handler = move |hovered| (handler.clone())(hovered);
        let mouse_leave_handler = mouse_enter_handler.clone();
        self.update_raw_el(|raw_el| {
            raw_el
                .event_handler(move |_: events::MouseEnter| mouse_enter_handler(true))
                .event_handler(move |_: events::MouseLeave| mouse_leave_handler(false))
        })
    }
}

// ------ Hookable ------

pub trait Hookable<T: RawEl>: UpdateRawEl<T> + Sized {
    type WSElement: JsCast;

    fn after_insert(self, handler: impl FnOnce(Self::WSElement) + Clone + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.after_insert(|ws_element| handler(ws_element.unchecked_into()))
        })
    }

    fn after_remove(self, handler: impl FnOnce(Self::WSElement) + Clone + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.after_remove(|ws_element| handler(ws_element.unchecked_into()))
        })
    }
}

// ------ MutableViewport ------
use std::convert::TryFrom;

pub trait MutableViewport<T: RawEl>: UpdateRawEl<T> + Sized {
    fn on_viewport_location_change(self, handler: impl FnOnce(Scene, Viewport) + Clone + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el
                .event_handler(move |event: events::Scroll| {
                    let target = event.target().unwrap_throw().unchecked_into::<web_sys::Element>();
                    let scene = Scene {
                        width: u32::try_from(target.scroll_width()).unwrap_throw(),
                        height: u32::try_from(target.scroll_height()).unwrap_throw(),
                    };
                    let viewport = Viewport {
                        x: target.scroll_left(),
                        y: target.scroll_top(),
                        width: u32::try_from(target.client_width()).unwrap_throw(),
                        height: u32::try_from(target.client_height()).unwrap_throw(),
                    };
                    handler(scene, viewport);
                })
        })
    }

    fn signal_for_viewport_x(self, x: impl Signal<Item = i32> + Unpin + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.prop_signal("scrollLeft", x)
        })
    } 

    fn signal_for_viewport_y(self, y: impl Signal<Item = i32> + Unpin + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.prop_signal("scrollTop", y)
        })
    } 
}
