use crate::*;

pub trait Focusable: UpdateRawEl<RawHtmlEl> + Sized {
    fn focus(self) -> Self {
        self.update_raw_el(|raw| raw.focus())
    } 
}

pub trait Styleable<T: RawEl>: UpdateRawEl<T> + Sized {
    fn style<'a>(self, style: impl Style<'a>) -> Self {
        self.update_raw_el(|mut raw_el| {
            for (name, value) in style.into_css_props() {
                raw_el = raw_el.style(name, value);
            }
            raw_el
        })
    } 
}
