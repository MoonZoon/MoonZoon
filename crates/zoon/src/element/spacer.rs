use crate::*;
use std::iter;

// ------ ------
//   Element
// ------ ------

pub struct Spacer {
    el: El<el::ChildFlagNotSet, RawHtmlEl<web_sys::HtmlElement>>,
}

impl Spacer {
    fn new_el() -> El<el::ChildFlagNotSet, RawHtmlEl<web_sys::HtmlElement>> {
        El::new()
            .update_raw_el(|raw_el| raw_el.class("spacer"))
            .pointer_handling(PointerHandling::none())
    }

    pub fn fill() -> Self {
        Self {
            el: Self::new_el().s(Width::fill()).s(Height::fill()),
        }
    }

    pub fn growable() -> Self {
        Self {
            el: Self::new_el().s(Width::growable()).s(Height::growable()),
        }
    }
}

impl Element for Spacer {
    fn into_raw_element(self) -> RawElement {
        self.el.into_raw_element()
    }
}

impl IntoIterator for Spacer {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl UpdateRawEl for Spacer {
    type RawEl = RawHtmlEl<web_sys::HtmlElement>;

    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.el = self.el.update_raw_el(updater);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ResizableViewport for Spacer {}
impl Hookable for Spacer {}
impl AddNearbyElement<'_> for Spacer {}
impl HasIds for Spacer {}
