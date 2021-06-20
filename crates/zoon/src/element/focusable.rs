use crate::*;

pub trait Focusable: UpdateRawEl<RawHtmlEl> + Sized {
    fn focus(self) -> Self {
        self.update_raw_el(|raw| raw.focus())
    } 
}
