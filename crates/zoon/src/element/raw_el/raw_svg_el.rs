use super::class_id_generator;
use crate::css_property::{CssPropertyName, CssPropertyValue};
use crate::*;
use std::{iter, marker::PhantomData};

// ------ ------
//   Element
// ------ ------

pub struct RawSvgEl<DomElement = web_sys::SvgElement>
    where web_sys::SvgElement: From<DomElement>
{
    class_id: ClassId,
    dom_builder: DomBuilder<web_sys::SvgElement>,
    dom_element_phantom: PhantomData<DomElement>,
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

impl<DomElement> RawSvgEl<DomElement> 
where web_sys::SvgElement: From<DomElement>
{
    pub fn dom_element_type<T>(self) -> RawSvgEl<T> 
        where web_sys::SvgElement: From<T>
    {
        RawSvgEl { class_id: self.class_id, dom_builder: self.dom_builder, dom_element_phantom: PhantomData }
    }
}

impl<DomElement> From<RawSvgEl<DomElement>> for RawElement 
where web_sys::SvgElement: From<DomElement>
{
    fn from(raw_svg_el: RawSvgEl<DomElement>) -> Self {
        RawElement::SvgEl(raw_svg_el.dom_element_type::<web_sys::SvgElement>())
    }
}

impl<DomElement> IntoDom for RawSvgEl<DomElement> 
where web_sys::SvgElement: From<DomElement>
{
    fn into_dom(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

impl<DomElement> Element for RawSvgEl<DomElement> 
where web_sys::SvgElement: From<DomElement>
{
    fn into_raw_element(self) -> RawElement {
        RawElement::SvgEl(self.dom_element_type::<web_sys::SvgElement>())
    }
}

impl<DomElement> IntoIterator for RawSvgEl<DomElement> 
where web_sys::SvgElement: From<DomElement>
{
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

impl<DomElement> RawEl for RawSvgEl<DomElement> 
    where
        DomElement:JsCast,
        web_sys::SvgElement: From<DomElement>
{
    type DomBuilderElement = web_sys::SvgElement;
    type DomElement = DomElement;

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
            dom_element_phantom: PhantomData,
        }
    }
}
