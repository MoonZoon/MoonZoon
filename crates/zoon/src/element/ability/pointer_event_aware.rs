use crate::*;

// ------ PointerEventAware ------

pub trait PointerEventAware<T: RawEl>: UpdateRawEl<T> + Sized {
    // fn on_hovered_change(self, handler: impl FnOnce(bool) + Clone + 'static) -> Self {
    //     let mouse_over_handler = move |hovered| (handler.clone())(hovered);
    //     let mouse_leave_handler = mouse_over_handler.clone();
    //     self.update_raw_el(|raw_el| {
    //         raw_el
    //             .event_handler(move |_: events_extra::MouseOver| mouse_over_handler(true))
    //             .event_handler(move |_: events::MouseLeave| mouse_leave_handler(false))
    //     })
    // }

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

    // @TODO https://developer.mozilla.org/en-US/docs/Web/CSS/pointer-events

    // fn on_double_click(self, handler: impl FnOnce() + Clone + 'static) -> Self {
    //     let handler = move || handler.clone()();
    //     self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::DoubleClick| handler()))
    // }

    // fn on_click_outside(
    //     self,
    //     handler: impl FnOnce() + Clone + 'static,
    //     ignored_class_ids: impl IntoIterator<Item = ClassId>,
    // ) -> Self {
    //     let handler = move || handler.clone()();
    //     let mut ignored_class_ids = ignored_class_ids.into_iter().collect::<Vec<_>>();

    //     self.update_raw_el(move |raw_el| {
    //         ignored_class_ids.push(raw_el.class_id());

    //         raw_el.global_event_handler(move |event: events::Click| {
    //             let selector = ignored_class_ids
    //                 .iter()
    //                 .filter_map(|class_id| {
    //                     class_id.map(|option_class_id| Some([".", &option_class_id?].concat()))
    //                 })
    //                 .collect::<Vec<_>>()
    //                 .join(", ");

    //             if closest(event.target(), &selector).is_none() {
    //                 handler()
    //             }
    //         })
    //     })
    // }
}

// fn closest(target: Option<web_sys::EventTarget>, selector: &str) -> Option<web_sys::Element> {
//     target?
//         .dyn_ref::<web_sys::Element>()?
//         .closest(selector)
//         .ok()?
// }

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
    PointerDown(events_extra::PointerDown)
}
