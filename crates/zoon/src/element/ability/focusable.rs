use crate::*;

pub trait Focusable: UpdateRawEl<RawHtmlEl> + Sized {
    fn focus(self, focus: bool) -> Self {
        if focus {
            return self.update_raw_el(|raw_el| raw_el.focused());
        }
        self
    }

    fn on_focus(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::Focus| handler()))
    }

    fn on_blur(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::Blur| handler()))
    }

    fn on_focused_change(self, handler: impl FnOnce(bool) + Clone + 'static) -> Self {
        let focus_handler = move |focused| (handler.clone())(focused);
        let blur_handler = focus_handler.clone();
        self.on_focus(move || focus_handler(true))
            .on_blur(move || blur_handler(false))
    }
}
