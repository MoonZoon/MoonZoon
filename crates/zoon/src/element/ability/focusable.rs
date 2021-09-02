use crate::*;

pub trait Focusable: UpdateRawEl<RawHtmlEl> + Sized {
    fn focus(self, focus: bool) -> Self {
        if focus {
            return self.update_raw_el(|raw_el| raw_el.focused())
        }
        self
    }

    fn on_blur(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::Blur| handler()))
    }
}
