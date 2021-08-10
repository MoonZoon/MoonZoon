use crate::*;
use std::borrow::Borrow;
use std::convert::TryFrom;

// ------ Focusable ------

pub trait Focusable: UpdateRawEl<RawHtmlEl> + Sized {
    fn focused(self) -> Self {
        self.update_raw_el(|raw_el| raw_el.focused())
    }

    fn on_blur(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |_: events::Blur| handler())
        })
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

// ------ MouseEventAware ------

pub trait MouseEventAware<T: RawEl>: UpdateRawEl<T> + Sized {
    fn on_hovered_change(self, handler: impl FnOnce(bool) + Clone + 'static) -> Self {
        let mouse_over_handler = move |hovered| (handler.clone())(hovered);
        let mouse_leave_handler = mouse_over_handler.clone();
        self.update_raw_el(|raw_el| {
            raw_el
                .event_handler(move |_: events_extra::MouseOver| mouse_over_handler(true))
                .event_handler(move |_: events::MouseLeave| mouse_leave_handler(false))
        })
    }

    fn on_click(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |_: events::Click| handler())
        })
    }

    fn on_double_click(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |_: events::DoubleClick| handler())
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

pub trait MutableViewport<T: RawEl>: UpdateRawEl<T> + Sized {
    fn on_viewport_location_change(
        self,
        handler: impl FnOnce(Scene, Viewport) + Clone + 'static,
    ) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |event: events::Scroll| {
                let target = event
                    .target()
                    .unwrap_throw()
                    .unchecked_into::<web_sys::Element>();
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

    fn viewport_x_signal(self, x: impl Signal<Item = i32> + Unpin + 'static) -> Self {
        self.update_raw_el(|raw_el| raw_el.prop_signal("scrollLeft", x))
    }

    fn viewport_y_signal(self, y: impl Signal<Item = i32> + Unpin + 'static) -> Self {
        self.update_raw_el(|raw_el| raw_el.prop_signal("scrollTop", y))
    }
}

// ------ AddNearbyElement ------

pub trait AddNearbyElement<'a>: UpdateRawEl<RawHtmlEl> + Sized {
    fn element_above(self, element: impl IntoOptionElement<'a> + 'a) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el
                .child(
                    RawHtmlEl::new("div")
                        .style("position", "absolute")
                        .style("bottom", "100%")
                        .style("left", "0")
                        .style("width", "100%")
                        .style("pointer-events", "none")
                        .attr("class", "above")
                        .child(element)
                )
        })
    }

    fn element_below(self, element: impl IntoOptionElement<'a> + 'a) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el
                .child(
                    RawHtmlEl::new("div")
                        .style("position", "absolute")
                        .style("top", "100%")
                        .style("left", "0")
                        .style("width", "100%")
                        .style("pointer-events", "none")
                        .attr("class", "below")
                        .child(element)
                )
        })
    }

    fn element_on_left(self, element: impl IntoOptionElement<'a> + 'a) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el
                .child(
                    RawHtmlEl::new("div")
                        .style("position", "absolute")
                        .style("right", "100%")
                        .style("top", "0")
                        .style("height", "100%")
                        .style("pointer-events", "none")
                        .attr("class", "on_left")
                        .child(element)
                )
        })
    }

    fn element_on_right(self, element: impl IntoOptionElement<'a> + 'a) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el
                .child(
                    RawHtmlEl::new("div")
                        .style("position", "absolute")
                        .style("left", "100%")
                        .style("top", "0")
                        .style("height", "100%")
                        .style("pointer-events", "none")
                        .attr("class", "on_right")
                        .child(element)
                )
        })
    }

    fn element_on_right_signal(
        self, 
        element: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static
    ) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el
                .child(
                    RawHtmlEl::new("div")
                        .style("position", "absolute")
                        .style("left", "100%")
                        .style("top", "0")
                        .style("height", "100%")
                        .style("pointer-events", "none")
                        .attr("class", "on_right")
                        .child_signal(element)
                )
        })
    }
}
