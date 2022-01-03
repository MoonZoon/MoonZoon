use super::class_id_generator;
use crate::css_property::{CssPropertyName, CssPropertyValue};
use crate::*;
use std::iter;

// ------ ------
//   Element
// ------ ------

pub struct RawSvgEl {
    class_id: ClassId,
    dom_builder: DomBuilder<web_sys::SvgElement>,
}

impl RawSvgEl {
    pub fn new(tag: &str) -> Self {
        let class_id = class_id_generator().next_class_id();

        let mut dom_builder = DomBuilder::new_svg(tag);
        dom_builder = class_id.map(move |class_id| dom_builder.class(class_id.unwrap_throw()));

        Self {
            class_id: class_id.clone(),
            dom_builder: dom_builder
                .after_removed(move |_| class_id_generator().remove_class_id(class_id)),
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

impl IntoIterator for RawSvgEl {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

// ------ ------
//  Attributes
// ------ ------

impl RawEl for RawSvgEl {
    type WSElement = web_sys::SvgElement;

    fn update_dom_builder(
        mut self,
        updater: impl FnOnce(DomBuilder<Self::WSElement>) -> DomBuilder<Self::WSElement>,
    ) -> Self {
        self.dom_builder = updater(self.dom_builder);
        self
    }

    fn style(self, name: &str, value: &str) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.style(CssPropertyName::new(name), CssPropertyValue::new(value))
        })
    }

    fn style_important(self, name: &str, value: &str) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.style_important(CssPropertyName::new(name), CssPropertyValue::new(value))
        })
    }

    fn style_signal<'a>(
        self,
        name: impl IntoCowStr<'static>,
        value: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
    ) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.style_signal(
                name.into_cow_str_wrapper().into_css_property_name(),
                value.map(|value| {
                    value
                        .into_option_cow_str_wrapper()
                        .map(|cow_str| cow_str.into_css_property_value())
                }),
            )
        })
    }

    fn class_id(&self) -> ClassId {
        self.class_id.clone()
    }
}
