use crate::*;

pub trait Styleable<'a, T: RawEl>: UpdateRawEl<T> + Sized {
    fn s(self, style: impl Style<'a>) -> Self {
        self.update_raw_el(|raw_el| style.update_raw_el_style(raw_el))
    }
}
