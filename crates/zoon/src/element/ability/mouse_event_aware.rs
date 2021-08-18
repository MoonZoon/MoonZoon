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

    fn on_click(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::Click| handler()))
    }

    fn on_double_click(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::DoubleClick| handler()))
    }
}
