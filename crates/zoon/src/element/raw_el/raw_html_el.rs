use super::class_id_generator;
use crate::css_property::{CssPropertyName, CssPropertyValue};
use crate::*;
use std::iter;

// ------ ------
//   Element
// ------ ------

pub struct RawHtmlEl {
    class_id: ClassId,
    dom_builder: DomBuilder<web_sys::HtmlElement>,
}

impl RawHtmlEl {
    pub fn new(tag: &str) -> Self {
        let class_id = class_id_generator().next_class_id();

        let mut dom_builder = DomBuilder::new_html(tag);
        dom_builder = class_id.map(move |class_id| dom_builder.class(class_id.unwrap_throw()));

        Self {
            class_id: class_id.clone(),
            dom_builder: dom_builder
                .after_removed(move |_| class_id_generator().remove_class_id(class_id)),
        }
    }
}

impl From<RawHtmlEl> for RawElement {
    fn from(raw_html_el: RawHtmlEl) -> Self {
        RawElement::El(raw_html_el)
    }
}

impl IntoDom for RawHtmlEl {
    fn into_dom(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

impl Element for RawHtmlEl {
    fn into_raw_element(self) -> RawElement {
        RawElement::El(self)
    }
}

impl IntoIterator for RawHtmlEl {
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

impl RawEl for RawHtmlEl {
    type DomBuilderElement = web_sys::HtmlElement;
    type DomElement = web_sys::HtmlElement;

    fn update_dom_builder(
        mut self,
        updater: impl FnOnce(DomBuilder<Self::DomBuilderElement>) -> DomBuilder<Self::DomBuilderElement>,
    ) -> Self {
        self.dom_builder = updater(self.dom_builder);
        self
    }

    fn dom_builder_element(&self) -> Self::DomBuilderElement {
        self.dom_builder.__internal_element()
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

    fn from_dom_element(dom_element: Self::DomElement) -> Self {
        let dom_builder_element = Self::DomBuilderElement::from(dom_element);
        let mut dom_builder = DomBuilder::new(dom_builder_element);

        let class_id = class_id_generator().next_class_id();
        dom_builder = class_id.map(move |class_id| dom_builder.class(class_id.unwrap_throw()));

        Self {
            class_id: class_id.clone(),
            dom_builder: dom_builder
                .after_removed(move |_| class_id_generator().remove_class_id(class_id)),
        }
    }
}

impl RawHtmlEl {
    pub fn focus(mut self) -> Self {
        self.dom_builder = self.dom_builder.focused(true);
        self
    }

    pub fn focus_signal(self, focus: impl Signal<Item = bool> + Unpin + 'static) -> Self {
        self.update_dom_builder(|dom_builder| dom_builder.focused_signal(focus))
    }
}
