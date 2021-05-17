use crate::*;

// ------ ------
//   Element
// ------ ------

pub struct RawText {
    dom: Dom,
}

impl RawText {
    pub fn new(text: impl AsRef<str>) -> Self {
        Self {
            dom: dominator::text(text.as_ref()),
        }
    }

    pub fn with_signal<'a>(
        text: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    ) -> Self {
        Self {
            dom: dominator::text_signal(text.map(|text| text.into_cow_str_wrapper())),
        }
    }
}

impl From<RawText> for RawElement {
    fn from(raw_text: RawText) -> Self {
        RawElement::Text(raw_text)
    }
}

impl IntoDom for RawText {
    fn into_dom(self) -> Dom {
        self.dom
    }
}

impl Element for RawText {
    fn into_raw_element(self) -> RawElement {
        self.into()
    }
}
