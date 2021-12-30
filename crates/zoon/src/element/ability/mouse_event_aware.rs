use crate::*;

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

    // @TODO add `on_click_event`
    fn on_click(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::Click| handler()))
    }

    // @TODO add `on_double_click_event`
    fn on_double_click(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::DoubleClick| handler()))
    }

    fn on_click_outside(
        self,
        handler: impl FnOnce() + Clone + 'static,
        ignored_class_ids: impl IntoIterator<Item = ClassId>,
    ) -> Self {
        let handler = move || handler.clone()();
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
                    handler()
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
