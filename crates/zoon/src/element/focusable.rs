use crate::*;

pub trait Focusable: UpdateRawHtmlEl + Sized {
    fn focus(self) -> Self {
        self.update_raw_html_el(|raw| raw.focus())
    } 
}
