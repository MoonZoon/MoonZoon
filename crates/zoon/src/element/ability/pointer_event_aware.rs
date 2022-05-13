use crate::*;
use std::{cell::Cell, rc::Rc};

// ------ PointerEventAware ------

pub trait PointerEventAware: UpdateRawEl + Sized {
    fn on_pointer_down(self, mut handler: impl FnMut() + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |_: events_extra::PointerDown| handler())
        })
    }

    fn on_pointer_down_event(self, mut handler: impl FnMut(PointerEvent) + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |event: events_extra::PointerDown| {
                let pointer_event = PointerEvent {
                    x: event.x(),
                    y: event.y(),
                    movement_x: 0,
                    movement_y: 0,
                    raw_event: RawPointerEvent::PointerDown(event),
                };
                handler(pointer_event)
            })
        })
    }

    fn on_pointer_up(self, mut handler: impl FnMut() + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |_: events_extra::PointerUp| handler())
        })
    }

    fn on_pointer_up_event(self, mut handler: impl FnMut(PointerEvent) + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |event: events_extra::PointerUp| {
                let pointer_event = PointerEvent {
                    x: event.x(),
                    y: event.y(),
                    movement_x: 0,
                    movement_y: 0,
                    raw_event: RawPointerEvent::PointerUp(event),
                };
                handler(pointer_event)
            })
        })
    }

    fn on_pointer_move(self, mut handler: impl FnMut() + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |_: events_extra::PointerMove| handler())
        })
    }

    fn on_pointer_move_event(self, mut handler: impl FnMut(PointerEvent) + 'static) -> Self {
        // `event.movement_*()` fails on iOS / touch screens (?)
        let previous_x = Rc::new(Cell::new(None));
        let previous_y = Rc::new(Cell::new(None));
        self.on_pointer_down({
            let previous_x = Rc::clone(&previous_x);
            let previous_y = Rc::clone(&previous_y);
            move || {
                previous_x.take();
                previous_y.take();
            }
        })
        .update_raw_el(|raw_el| {
            raw_el.event_handler(move |event: events_extra::PointerMove| {
                let x = event.x();
                let y = event.y();
                let previous_x = previous_x.replace(Some(x));
                let previous_y = previous_y.replace(Some(y));
                let pointer_event = PointerEvent {
                    x: event.x(),
                    y: event.y(),
                    movement_x: previous_x.map_or(0, |previous_x| x - previous_x),
                    movement_y: previous_y.map_or(0, |previous_y| y - previous_y),
                    raw_event: RawPointerEvent::PointerMove(event),
                };
                handler(pointer_event)
            })
        })
    }

    fn on_pointer_leave(self, mut handler: impl FnMut() + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            let dom_element = raw_el.dom_element().into();
            raw_el.event_handler(move |event: events_extra::PointerLeave| {
                if let Some(target) = event.target() {
                    // we are leaving from the element itself, not only from its child
                    if target == dom_element {
                        handler();
                    }
                }
            })
        })
    }

    fn pointer_handling(self, handling: PointerHandling) -> Self
    where
        <Self::RawEl as RawEl>::DomElement: Into<web_sys::HtmlElement>,
    {
        self.update_raw_el(|raw_el| raw_el.style("pointer-events", handling.pointer_events))
    }

    fn pointer_handling_signal(
        self,
        handling: impl Signal<Item = PointerHandling> + Unpin + 'static,
    ) -> Self
    where
        <Self::RawEl as RawEl>::DomElement: Into<web_sys::HtmlElement>,
    {
        self.update_raw_el(|raw_el| {
            raw_el.style_signal(
                "pointer-events",
                handling.map(|handling| handling.pointer_events),
            )
        })
    }

    fn pointer_handling_svg(self, handling: PointerHandlingSvg) -> Self
    where
        <Self::RawEl as RawEl>::DomElement: Into<web_sys::SvgElement>,
    {
        self.update_raw_el(|raw_el| raw_el.style("pointer-events", handling.pointer_events))
    }

    fn pointer_handling_svg_signal(
        self,
        handling: impl Signal<Item = PointerHandlingSvg> + Unpin + 'static,
    ) -> Self
    where
        <Self::RawEl as RawEl>::DomElement: Into<web_sys::SvgElement>,
    {
        self.update_raw_el(|raw_el| {
            raw_el.style_signal(
                "pointer-events",
                handling.map(|handling| handling.pointer_events),
            )
        })
    }
}

// ------ PointerEvent ------

pub struct PointerEvent {
    x: i32,
    y: i32,
    movement_x: i32,
    movement_y: i32,
    pub raw_event: RawPointerEvent,
}

impl PointerEvent {
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn movement_x(&self) -> i32 {
        self.movement_x
    }

    pub fn movement_y(&self) -> i32 {
        self.movement_y
    }
}

// ------ RawPointerEvent ------

pub enum RawPointerEvent {
    PointerDown(events_extra::PointerDown),
    PointerUp(events_extra::PointerUp),
    PointerMove(events_extra::PointerMove),
}

// ------ PointerHandling ------

#[derive(Clone, Copy)]
pub struct PointerHandling {
    pointer_events: &'static str,
}

impl Default for PointerHandling {
    fn default() -> Self {
        Self {
            pointer_events: "auto",
        }
    }
}

impl PointerHandling {
    pub fn none() -> Self {
        Self {
            pointer_events: "none",
        }
    }
}

// ------ PointerHandlingSvg ------

#[derive(Clone, Copy)]
pub struct PointerHandlingSvg {
    pointer_events: &'static str,
}

impl Default for PointerHandlingSvg {
    fn default() -> Self {
        Self {
            pointer_events: "auto",
        }
    }
}

impl PointerHandlingSvg {
    pub fn none() -> Self {
        Self {
            pointer_events: "none",
        }
    }

    pub fn visible_fill() -> Self {
        Self {
            pointer_events: "visibleFill",
        }
    }

    pub fn fill() -> Self {
        Self {
            pointer_events: "fill",
        }
    }

    pub fn visible_stroke() -> Self {
        Self {
            pointer_events: "visibleStroke",
        }
    }

    pub fn stroke() -> Self {
        Self {
            pointer_events: "stroke",
        }
    }

    pub fn painted() -> Self {
        Self {
            pointer_events: "painted",
        }
    }

    pub fn all() -> Self {
        Self {
            pointer_events: "all",
        }
    }
}
