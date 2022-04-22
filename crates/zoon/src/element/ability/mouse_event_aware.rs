use crate::*;

// ------ MouseEventAware ------

pub trait MouseEventAware: UpdateRawEl + Sized {
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
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::Click| handler()))
    }

    fn on_click_event(self, handler: impl FnOnce(MouseEvent) + Clone + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |event: events::Click| {
                let mouse_event = MouseEvent {
                    x: event.x(),
                    y: event.y(),
                    movement_x: 0,
                    movement_y: 0,
                    raw_event: RawMouseEvent::Click(event),
                };
                (handler.clone())(mouse_event)
            })
        })
    }

    fn on_double_click(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::DoubleClick| handler()))
    }

    fn on_double_click_event(self, handler: impl FnOnce(MouseEvent) + Clone + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |event: events::DoubleClick| {
                let mouse_event = MouseEvent {
                    x: event.x(),
                    y: event.y(),
                    movement_x: 0,
                    movement_y: 0,
                    raw_event: RawMouseEvent::DoubleClick(event),
                };
                (handler.clone())(mouse_event)
            })
        })
    }

    fn on_click_outside<'a>(
        self,
        handler: impl FnOnce() + Clone + 'static,
        ignored_ids: impl IntoIterator<Item = impl IntoCowStr<'a>>,
    ) -> Self {
        let handler = move || handler.clone()();
        let ids_selector = ignored_ids
            .into_iter()
            .map(|id| crate::format!("#{}", id.into_cow_str()))
            .collect::<Vec<_>>()
            .join(", ");

        self.update_raw_el(move |raw_el| {
            let dom_element = raw_el.dom_element().unchecked_into::<web_sys::Element>();
            raw_el.global_event_handler(move |event: events::Click| {
                let target = event.target().expect_throw("failed to get event target");
                if dom_element.contains(Some(target.unchecked_ref())) {
                    return;
                }
                if dom_element.closest(&ids_selector).expect_throw("failed to get closest elements").is_some() {
                    return;
                }
                handler();
            })
        })
    }

    fn on_click_outside_event(
        self,
        handler: impl FnOnce(MouseEvent) + Clone + 'static,
        ignored_class_ids: impl IntoIterator<Item = ClassId>,
    ) -> Self {
        let mut ignored_class_ids = ignored_class_ids.into_iter().collect::<Vec<_>>();

        self.update_raw_el(move |raw_el| {
            ignored_class_ids.push(raw_el.class_id());

            raw_el.global_event_handler(move |event: events::Click| {
                let selector = ignored_class_ids
                    .iter()
                    .filter_map(|class_id| {
                        class_id.map(|option_class_id| Some([".", &option_class_id?].concat()))
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                if closest(event.target(), &selector).is_none() {
                    let mouse_event = MouseEvent {
                        x: event.x(),
                        y: event.y(),
                        movement_x: 0,
                        movement_y: 0,
                        raw_event: RawMouseEvent::Click(event),
                    };
                    (handler.clone())(mouse_event)
                }
            })
        })
    }
}

fn closest(target: Option<web_sys::EventTarget>, selector: &str) -> Option<web_sys::Element> {
    target?
        .dyn_ref::<web_sys::Element>()?
        .closest(selector)
        .ok()?
}

// ------ MouseEvent ------

pub struct MouseEvent {
    x: i32,
    y: i32,
    movement_x: i32,
    movement_y: i32,
    pub raw_event: RawMouseEvent,
}

impl MouseEvent {
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

// ------ RawMouseEvent ------

pub enum RawMouseEvent {
    Click(events::Click),
    DoubleClick(events::DoubleClick),
}
