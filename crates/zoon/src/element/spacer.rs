use crate::*;

// ------ ------
//   Element
// ------ ------

pub struct Spacer {
    raw_el: RawHtmlEl<web_sys::HtmlElement>,
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
            raw_el: Self::new_el().s(Width::fill()).s(Height::fill()).into_raw_el(),
        }
    }

    #[track_caller]
    pub fn growable() -> Self {
        Self {
            raw_el: Self::new_el().s(Width::growable()).s(Height::growable()).into_raw_el(),
        }
    }

    #[track_caller]
    pub fn growable_with_factor<T: Into<f64>>(factor: impl Into<Option<T>>) -> Self {
        if let Some(factor) = factor.into() {
            let factor = factor.into();
            Self {
                raw_el: Self::new_el()
                    .s(Width::growable_with_factor(factor))
                    .s(Height::growable_with_factor(factor))
                    .into_raw_el()
            }
        } else {
            Self::growable()
        }
    }
}

impl Element for Spacer {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into_raw_element()
    }
}

impl RawElWrapper for Spacer {
    type RawEl = RawHtmlEl<web_sys::HtmlElement>;

    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ResizableViewport for Spacer {}
impl Hookable for Spacer {}
impl AddNearbyElement<'_> for Spacer {}
impl HasIds for Spacer {}
