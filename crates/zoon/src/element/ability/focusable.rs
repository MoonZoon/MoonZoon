use crate::*;

pub trait Focusable: UpdateRawEl<RawHtmlEl> + Sized {
    fn focused(self) -> Self {
        self.update_raw_el(|raw_el| raw_el.focused())
    }

    fn on_blur(self, handler: impl FnOnce() + Clone + 'static) -> Self {
        let handler = move || handler.clone()();
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |_: events::Blur| handler())
        })
    }
}
