use crate::*;

pub trait Hookable<T: RawEl>: UpdateRawEl<T> + Sized {
    type WSElement: JsCast;

    fn after_insert(self, handler: impl FnOnce(Self::WSElement) + Clone + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.after_insert(|ws_element| handler(ws_element.unchecked_into()))
        })
    }

    fn after_remove(self, handler: impl FnOnce(Self::WSElement) + Clone + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.after_remove(|ws_element| handler(ws_element.unchecked_into()))
        })
    }
}
