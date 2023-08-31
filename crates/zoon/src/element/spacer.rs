use crate::*;

// ------ ------
//   Element
// ------ ------

pub struct Spacer {
    el: El<el::ChildFlagNotSet, RawHtmlEl<web_sys::HtmlElement>>,
}

impl Spacer {
    #[track_caller]
    fn new_el() -> El<el::ChildFlagNotSet, RawHtmlEl<web_sys::HtmlElement>> {
        El::new()
            .update_raw_el(|raw_el| raw_el.class("spacer"))
            .pointer_handling(PointerHandling::none())
    }

    #[track_caller]
    pub fn fill() -> Self {
        Self {
            el: Self::new_el().s(Width::fill()).s(Height::fill()),
        }
    }

    #[track_caller]
    pub fn growable() -> Self {
        Self {
            el: Self::new_el().s(Width::growable()).s(Height::growable()),
        }
    }

    #[track_caller]
    pub fn growable_with_factor<T: Into<f64>>(factor: impl Into<Option<T>>) -> Self {
        if let Some(factor) = factor.into() {
            let factor = factor.into();
            Self {
                el: Self::new_el()
                    .s(Width::growable_with_factor(factor))
                    .s(Height::growable_with_factor(factor)),
            }
        } else {
            Self::growable()
        }
    }
}

impl Element for Spacer {
    fn into_raw_element(self) -> RawElement {
        self.el.into_raw_element()
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
