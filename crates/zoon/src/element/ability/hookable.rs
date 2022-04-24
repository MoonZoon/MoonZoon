use crate::*;

pub trait Hookable: UpdateRawEl + Sized {
    fn after_insert(
        self,
        handler: impl FnOnce(<Self::RawEl as RawEl>::DomElement) + 'static,
    ) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.after_insert(handler)
        })
    }

    fn after_remove(
        self,
        handler: impl FnOnce(<Self::RawEl as RawEl>::DomElement) + 'static,
    ) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.after_remove(handler)
        })
    }
}
