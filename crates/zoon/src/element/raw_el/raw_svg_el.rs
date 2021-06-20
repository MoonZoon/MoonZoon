use crate::*;

// ------ ------
//   Element
// ------ ------

pub struct RawSvgEl {
    dom_builder: DomBuilder<web_sys::SvgElement>,
}

impl RawSvgEl {
    pub fn new(tag: &str) -> Self {
        Self {
            dom_builder: DomBuilder::new_svg(tag),
        }
    }
}

impl From<RawSvgEl> for RawElement {
    fn from(raw_svg_el: RawSvgEl) -> Self {
        RawElement::SvgEl(raw_svg_el)
    }
}

impl IntoDom for RawSvgEl {
    fn into_dom(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

impl Element for RawSvgEl {
    fn into_raw_element(self) -> RawElement {
        RawElement::SvgEl(self)
    }
}

// ------ ------
//  Attributes
// ------ ------

impl RawEl for RawSvgEl {
    type WSElement = web_sys::SvgElement;

    fn update_dom_builder(mut self, updater: impl FnOnce(DomBuilder<Self::WSElement>) -> DomBuilder<Self::WSElement>) -> Self {
        self.dom_builder = updater(self.dom_builder);
        self
    }
}
