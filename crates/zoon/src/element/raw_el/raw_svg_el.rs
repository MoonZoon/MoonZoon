use super::class_id_generator;
use crate::css_property::{CssPropertyName, CssPropertyValue};
use crate::*;
use std::{iter, marker::PhantomData};

// ------ ------
//   Element
// ------ ------

pub struct RawSvgEl<WSElement = web_sys::SvgElement> {
    class_id: ClassId,
    dom_builder: DomBuilder<web_sys::SvgElement>,
    dom_element_phantom: PhantomData<WSElement>,
}

impl RawSvgEl<web_sys::SvgElement> {
    pub fn new(tag: &str) -> Self {
        let class_id = class_id_generator().next_class_id();

        let mut dom_builder = DomBuilder::new_svg(tag);
        dom_builder = class_id.map(move |class_id| dom_builder.class(class_id.unwrap_throw()));

        Self {
            class_id: class_id.clone(),
            dom_builder: dom_builder
                .after_removed(move |_| class_id_generator().remove_class_id(class_id)),
            dom_element_phantom: PhantomData,
        }
    }
}

impl<WSElement> RawSvgEl<WSElement> {
    pub fn dom_element_type<T: AsRef<web_sys::SvgElement>>(self) -> RawSvgEl<T> {
        RawSvgEl { class_id: self.class_id, dom_builder: self.dom_builder, dom_element_phantom: PhantomData }
    }
}

impl<WSElement> From<RawSvgEl<WSElement>> for RawElement {
    fn from(raw_svg_el: RawSvgEl<WSElement>) -> Self {
        RawElement::SvgEl(raw_svg_el.dom_element_type::<web_sys::SvgElement>())
    }
}

impl<WSElement> IntoDom for RawSvgEl<WSElement> {
    fn into_dom(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

impl<WSElement: AsRef<web_sys::SvgElement>> Element for RawSvgEl<WSElement> {
    fn into_raw_element(self) -> RawElement {
        RawElement::SvgEl(self.dom_element_type::<web_sys::SvgElement>())
    }
}

impl<WSElement: AsRef<web_sys::SvgElement>> IntoIterator for RawSvgEl<WSElement> {
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

impl<WSElement> RawEl for RawSvgEl<WSElement> 
    where
    WSElement: 
        Into<web_sys::Element> 
        + AsRef<web_sys::Element>
        + JsCast
{
    type WSElement = web_sys::SvgElement;
    type DomElement = WSElement;

    fn update_dom_builder(
        mut self,
        updater: impl FnOnce(DomBuilder<Self::WSElement>) -> DomBuilder<Self::WSElement>,
    ) -> Self {
        self.dom_builder = updater(self.dom_builder);
        self
    }

    fn dom_element(&self) -> Self::DomElement {
        self.dom_builder.__internal_element().unchecked_into()
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

    fn from_dom_element(dom_element: Self::WSElement) -> Self {
        let mut dom_builder = DomBuilder::new(dom_element);

        let class_id = class_id_generator().next_class_id();
        dom_builder = class_id.map(move |class_id| dom_builder.class(class_id.unwrap_throw()));

        Self {
            class_id: class_id.clone(),
            dom_builder: dom_builder
                .after_removed(move |_| class_id_generator().remove_class_id(class_id)),
            dom_element_phantom: PhantomData,
        }
    }
}
