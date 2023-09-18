use crate::*;

pub enum RawElOrText {
    RawHtmlEl(RawHtmlEl<web_sys::HtmlElement>),
    RawSvgEl(RawSvgEl<web_sys::SvgElement>),
    RawText(RawText),
}

impl ElementUnchecked for RawElOrText {
    fn into_raw_unchecked(self) -> Self {
        self
    }
}

impl Element for RawElOrText {}

impl IntoDom for RawElOrText {
    fn into_dom(self) -> Dom {
        match self {
            Self::RawHtmlEl(raw_el) => raw_el.into_dom(),
            Self::RawSvgEl(raw_svg_el) => raw_svg_el.into_dom(),
            Self::RawText(raw_text) => raw_text.into_dom(),
        }
    }
}
