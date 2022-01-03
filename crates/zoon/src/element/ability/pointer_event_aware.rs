use crate::*;
use std::{cell::Cell, rc::Rc};

// ------ PointerEventAware ------

pub trait PointerEventAware<T: RawEl>: UpdateRawEl<T> + Sized {
    fn on_pointer_down(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events_extra::PointerDown| handler()))
    }

    fn on_pointer_down_event(self, handler: impl FnOnce(PointerEvent) + Clone + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |event: events_extra::PointerDown| {
                let pointer_event = PointerEvent {
                    x: event.x(),
                    y: event.y(),
                    movement_x: 0,
                    movement_y: 0,
                    raw_event: RawPointerEvent::PointerDown(event),
                };
                (handler.clone())(pointer_event)
            })
        })
    }

    fn on_pointer_up(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events_extra::PointerUp| handler()))
    }

    fn on_pointer_up_event(self, handler: impl FnOnce(PointerEvent) + Clone + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |event: events_extra::PointerUp| {
                let pointer_event = PointerEvent {
                    x: event.x(),
                    y: event.y(),
                    movement_x: 0,
                    movement_y: 0,
                    raw_event: RawPointerEvent::PointerUp(event),
                };
                (handler.clone())(pointer_event)
            })
        })
    }

    fn on_pointer_move(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events_extra::PointerMove| handler()))
    }

    fn on_pointer_move_event(self, handler: impl FnOnce(PointerEvent) + Clone + 'static) -> Self {
        // `event.movement_*()` fails on iOS / touch screens (?)
        let previous_x = Rc::new(Cell::new(None));
        let previous_y = Rc::new(Cell::new(None));
        self
            .on_pointer_down({
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
                    (handler.clone())(pointer_event)
                })
            })
    }

    fn on_pointer_leave(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| {
            let class_id = raw_el.class_id();
            raw_el
                .event_handler(move |event: events_extra::PointerLeave| {
                    let target = event.dyn_target::<web_sys::Element>().unwrap_throw();
                    let classes = target.get_attribute("class").unwrap_throw();
                    class_id.map(|class_id| {
                        let class_id = class_id.unwrap_throw();
                        for class in classes.split_ascii_whitespace() {
                            if class == class_id {
                                handler();
                                return;
                            }
                        }
                    });
                })
        })
    }

    // @TODO https://developer.mozilla.org/en-US/docs/Web/CSS/pointer-events
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
