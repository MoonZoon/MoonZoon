use crate::*;

// ------ MouseEventAware ------

pub trait MouseEventAware: UpdateRawEl + Sized {
    fn on_hovered_change(self, handler: impl FnMut(bool) + 'static) -> Self {
        let mouse_over_handler = move |hovered| (handler.clone())(hovered);
        let mouse_leave_handler = mouse_over_handler.clone();
        self.update_raw_el(|raw_el| {
            raw_el
                .event_handler(move |_: events_extra::MouseOver| mouse_over_handler(true))
                .event_handler(move |_: events::MouseLeave| mouse_leave_handler(false))
        })
    }

    fn on_click(self, handler: impl FnMut() + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::Click| handler()))
    }

    fn on_click_event(self, handler: impl FnMut(MouseEvent) + 'static) -> Self {
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

    fn on_double_click(self, handler: impl FnMut() + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::DoubleClick| handler()))
    }

    fn on_double_click_event(self, handler: impl FnMut(MouseEvent) + 'static) -> Self {
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
        handler: impl FnMut() + 'static,
    ) -> Self {
        self.update_raw_el(move |raw_el| {
            let dom_element: web_sys::Element = raw_el.dom_element().into();
            raw_el.global_event_handler(move |event: events::Click| {
                if is_inside(&dom_element, &event, "") {
                    return;
                }
                (handler.clone())();
            })
        })
    }

    fn on_click_outside_event<'a>(
        self,
        handler: impl FnMut(MouseEvent) + 'static,
    ) -> Self {
        self.update_raw_el(move |raw_el| {
            let dom_element: web_sys::Element = raw_el.dom_element().into();
            raw_el.global_event_handler(move |event: events::Click| {
                if is_inside(&dom_element, &event, "") {
                    return;
                }
                let mouse_event = MouseEvent {
                    x: event.x(),
                    y: event.y(),
                    movement_x: 0,
                    movement_y: 0,
                    raw_event: RawMouseEvent::Click(event),
                };
                (handler.clone())(mouse_event);
            })
        })
    }

    fn on_click_outside_with_ids<'a>(
        self,
        handler: impl FnMut() + 'static,
        ignored_ids: impl IntoIterator<Item = impl IntoCowStr<'a>>,
    ) -> Self {
        let ids_selector = selector_from_ids(ignored_ids);
        self.update_raw_el(move |raw_el| {
            let dom_element: web_sys::Element = raw_el.dom_element().into();
            raw_el.global_event_handler(move |event: events::Click| {
                if is_inside(&dom_element, &event, &ids_selector) {
                    return;
                }
                (handler.clone())();
            })
        })
    }

    fn on_click_outside_with_ids_event<'a>(
        self,
        handler: impl FnMut(MouseEvent) + 'static,
        ignored_ids: impl IntoIterator<Item = impl IntoCowStr<'a>>,
    ) -> Self {
        let ids_selector = selector_from_ids(ignored_ids);
        self.update_raw_el(move |raw_el| {
            let dom_element: web_sys::Element = raw_el.dom_element().into();
            raw_el.global_event_handler(move |event: events::Click| {
                if is_inside(&dom_element, &event, &ids_selector) {
                    return;
                }
                let mouse_event = MouseEvent {
                    x: event.x(),
                    y: event.y(),
                    movement_x: 0,
                    movement_y: 0,
                    raw_event: RawMouseEvent::Click(event),
                };
                (handler.clone())(mouse_event);
            })
        })
    }
}

// -- Helpers --

fn selector_from_ids<'a>(ids: impl IntoIterator<Item = impl IntoCowStr<'a>>) -> String {
    ids
        .into_iter()
        .map(|id| crate::format!("#{}", id.into_cow_str()))
        .collect::<Vec<_>>()
        .join(", ")
}

fn is_inside(dom_element: &web_sys::Element, event: &events::Click, ids_selector: &str) -> bool {
    let target = event.target().expect_throw("failed to get event target");
    if dom_element.contains(Some(target.unchecked_ref())) {
        return true;
    }
    if not(ids_selector.is_empty()) && target
        .unchecked_ref::<web_sys::Element>()
        .closest(&ids_selector)
        .expect_throw("invalid selector provided")
        .is_some()
    {
        return true;
    }
    false
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
