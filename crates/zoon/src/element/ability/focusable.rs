use crate::*;
use std::{cell::RefCell, rc::Rc};

pub trait Focusable: UpdateRawEl + Sized
where
    <Self::RawEl as RawEl>::DomElement: AsRef<web_sys::HtmlElement>,
{
    fn focus(self, focus: bool) -> Self {
        if focus {
            return self.update_raw_el(|raw_el| raw_el.focus());
        }
        self
    }

    fn focus_signal(self, focus: impl Signal<Item = bool> + Unpin + 'static) -> Self {
        self.update_raw_el(|raw_el| raw_el.focus_signal(focus))
    }

    fn on_focus(self, mut handler: impl FnMut() + 'static) -> Self {
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::Focus| handler()))
    }

    fn on_blur(self, mut handler: impl FnMut() + 'static) -> Self {
        self.update_raw_el(|raw_el| raw_el.event_handler(move |_: events::Blur| handler()))
    }

    fn on_focused_change(self, handler: impl FnMut(bool) + 'static) -> Self {
        let focus_handler = Rc::new(RefCell::new(handler));
        let blur_handler = focus_handler.clone();
        self.on_focus(move || focus_handler.borrow_mut()(true))
            .on_blur(move || blur_handler.borrow_mut()(false))
    }
}
