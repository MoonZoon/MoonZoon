use crate::*;
use web_sys::{EventTarget, Node};

mod raw_html_el;
mod raw_svg_el;

pub use raw_html_el::RawHtmlEl;
pub use raw_svg_el::RawSvgEl;

pub trait RawEl {
    
}

impl RawEl for RawHtmlEl {}
impl RawEl for RawSvgEl {}

pub trait UpdateRawEl<T: RawEl> {
    fn update_raw_el(self, updater: impl FnOnce(T) -> T) -> Self;
}
