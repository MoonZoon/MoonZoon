use super::class_id_generator;
use crate::css_property::{CssPropertyName, CssPropertyValue};
use crate::*;
use std::iter;

// ------ ------
//   Element
// ------ ------

pub struct RawHtmlEl<DomElement: Into<web_sys::HtmlElement>> {
    class_id: ClassId,
    dom_builder: DomBuilder<DomElement>,
}

impl RawHtmlEl<web_sys::HtmlElement> {
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

impl<DomElement: Into<web_sys::HtmlElement> + Clone + JsCast> RawHtmlEl<DomElement> {
    pub fn dom_element_type<T: Into<web_sys::SvgElement> + JsCast>(self) -> RawHtmlEl<T> {
        let element = self.dom_builder.__internal_element().unchecked_into::<T>();
        let dom_builder = DomBuilder::new(element).__internal_transfer_callbacks(self.dom_builder);
        RawSvgEl { class_id: self.class_id, dom_builder }
    }
}

impl<DomElement: Into<web_sys::HtmlElement>> From<RawHtmlEl<DomElement>> for RawElement {
    fn from(raw_html_el: RawHtmlEl<DomElement>) -> Self {
        RawElement::El(raw_html_el)
    }
}

impl<DomElement: Into<web_sys::HtmlElement> + Into<web_sys::Node>> IntoDom for RawHtmlEl<DomElement> {
    fn into_dom(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

impl<DomElement: Into<web_sys::SvgElement> + Clone + JsCast> Element for RawHtmlEl<DomElement> {
    fn into_raw_element(self) -> RawElement {
        RawElement::El(self)
    }
}

impl<DomElement: Into<web_sys::HtmlElement>> IntoIterator for RawHtmlEl<DomElement> {
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

impl<DomElement> RawEl for RawHtmlEl<DomElement> 
    where DomElement: AsRef<web_sys::Node>
    + Into<web_sys::HtmlElement>
    + AsRef<web_sys::EventTarget>
    + AsRef<JsValue>
    + AsRef<web_sys::Element>
    + Into<web_sys::Element>
    + AsRef<web_sys::SvgElement>
    + Into<web_sys::Node>
    + Clone
    + JsCast
    + 'static
{
    type DomElement = web_sys::HtmlElement;

    fn update_dom_builder(
        mut self,
        updater: impl FnOnce(DomBuilder<Self::DomElement>) -> DomBuilder<Self::DomElement>,
    ) -> Self {
        self.dom_builder = updater(self.dom_builder);
        self
    }

    fn dom_element(&self) -> Self::DomElement {
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
        let mut dom_builder = DomBuilder::new(dom_element);

        let class_id = class_id_generator().next_class_id();
        dom_builder = class_id.map(move |class_id| dom_builder.class(class_id.unwrap_throw()));

        Self {
            class_id: class_id.clone(),
            dom_builder: dom_builder
                .after_removed(move |_| class_id_generator().remove_class_id(class_id)),
        }
    }
}

impl<DomElement> RawHtmlEl<DomElement> {
    pub fn focus(mut self) -> Self {
        self.dom_builder = self.dom_builder.focused(true);
        self
    }

    pub fn focus_signal(self, focus: impl Signal<Item = bool> + Unpin + 'static) -> Self {
        self.update_dom_builder(|dom_builder| dom_builder.focused_signal(focus))
    }
}
