use crate::*;

pub trait Hookable<T: RawEl>: UpdateRawEl<T> + Sized {
    fn after_insert(self, handler: impl FnOnce(T::DomElement) + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.after_insert(|ws_element| handler(ws_element.unchecked_into()))
        })
    }

    fn after_remove(self, handler: impl FnOnce(T::DomElement) + 'static) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.after_remove(|ws_element| handler(ws_element.unchecked_into()))
        })
    }
}
