use super::CLASS_ID_GENERATOR;
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
        <Self as RawEl>::new(tag)
    }
}

impl<DomElement: Into<web_sys::HtmlElement> + Clone + JsCast> RawHtmlEl<DomElement> {
    pub fn dom_element_type<T: Into<web_sys::HtmlElement> + JsCast>(self) -> RawHtmlEl<T> {
        let element = self.dom_builder.__internal_element().unchecked_into::<T>();
        let dom_builder = DomBuilder::new(element).__internal_transfer_callbacks(self.dom_builder);
        RawHtmlEl {
            class_id: self.class_id,
            dom_builder,
        }
    }
}

impl<DomElement: Into<web_sys::HtmlElement> + Clone + JsCast> From<RawHtmlEl<DomElement>>
    for RawElement
{
    fn from(raw_html_el: RawHtmlEl<DomElement>) -> Self {
        RawElement::El(raw_html_el.dom_element_type::<web_sys::HtmlElement>())
    }
}

impl<DomElement: Into<web_sys::HtmlElement> + Into<web_sys::Node>> IntoDom
    for RawHtmlEl<DomElement>
{
    fn into_dom(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

impl<DomElement: Into<web_sys::HtmlElement> + Clone + JsCast> Element for RawHtmlEl<DomElement> {
    fn into_raw_element(self) -> RawElement {
        RawElement::El(self.dom_element_type::<web_sys::HtmlElement>())
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
//     RawEl
// ------ ------

impl<DomElement> RawEl for RawHtmlEl<DomElement>
// Warning: "Global" bounds with `JsValue` or `JsCast` or `AsRef<web_sys::HtmlElement>` break Rust Analyzer.
where
    DomElement: AsRef<web_sys::Node>
        + AsRef<web_sys::EventTarget>
        + Into<web_sys::EventTarget>
        + AsRef<web_sys::Element>
        + Into<web_sys::Element>
        + Into<web_sys::HtmlElement>
        + Clone
        + 'static,
{
    type DomElement = DomElement;

    fn new(tag: &str) -> Self
    where
        DomElement: JsCast,
    {
        let class_id = CLASS_ID_GENERATOR.next_class_id();

        let mut dom_builder = DomBuilder::new_html(tag);
        dom_builder = class_id.map(move |class_id| dom_builder.class(class_id.unwrap_throw()));

        Self {
            class_id: class_id.clone(),
            dom_builder: dom_builder
                .after_removed(move |_| CLASS_ID_GENERATOR.remove_class_id(class_id)),
        }
    }

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

    fn class_id(&self) -> ClassId {
        self.class_id.clone()
    }

    fn from_dom_element(dom_element: Self::DomElement) -> Self {
        let mut dom_builder = DomBuilder::new(dom_element);

        let class_id = CLASS_ID_GENERATOR.next_class_id();
        dom_builder = class_id.map(move |class_id| dom_builder.class(class_id.unwrap_throw()));

        Self {
            class_id: class_id.clone(),
            dom_builder: dom_builder
                .after_removed(move |_| CLASS_ID_GENERATOR.remove_class_id(class_id)),
        }
    }

    fn focus(self) -> Self
    where
        Self::DomElement: AsRef<web_sys::HtmlElement>,
    {
        self.update_dom_builder(|dom_builder| dom_builder.focused(true))
    }

    fn focus_signal(self, focus: impl Signal<Item = bool> + Unpin + 'static) -> Self
    where
        Self::DomElement: AsRef<web_sys::HtmlElement>,
    {
        self.update_dom_builder(|dom_builder| dom_builder.focused_signal(focus))
    }
}
