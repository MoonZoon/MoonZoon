use crate::*;

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
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |event: events_extra::PointerMove| {
                let pointer_event = PointerEvent {
                    x: event.x(),
                    y: event.y(),
                    raw_event: RawPointerEvent::PointerMove(event),
                };
                (handler.clone())(pointer_event)
            })
        })
    }

    // @TODO https://developer.mozilla.org/en-US/docs/Web/CSS/pointer-events
}

// ------ PointerEvent ------

pub struct PointerEvent {
    x: i32,
    y: i32,
    pub raw_event: RawPointerEvent,
}

impl PointerEvent {
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

// ------ RawPointerEvent ------

pub enum RawPointerEvent {
    PointerDown(events_extra::PointerDown),
    PointerUp(events_extra::PointerUp),
    PointerMove(events_extra::PointerMove),
}
