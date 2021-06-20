use crate::*;

pub trait Styleable<T: RawEl>: UpdateRawEl<T> + Sized {
    fn style(self, style: impl Style) -> Self {
        self.update_raw_el(|raw_el| {
            // raw_el.st
            raw_el
        })
    } 
}
