use crate::*;
use web_sys::{EventTarget, Node};

// ------ ------
//   Element
// ------ ------

pub type RawHtmlEl = RawEl<web_sys::HtmlElement>;
pub type RawSvgEl = RawEl<web_sys::SvgElement>;

pub struct RawEl<T> {
    dom_builder: DomBuilder<T>,
}

impl RawHtmlEl {
    pub fn new(tag: &str) -> Self {
        Self {
            dom_builder: DomBuilder::new_html(tag),
        }
    }
}

impl RawSvgEl {
    pub fn new(tag: &str) -> Self {
        Self {
            dom_builder: DomBuilder::new_svg(tag),
        }
    }
}

impl From<RawHtmlEl> for RawElement {
    fn from(raw_html_el: RawHtmlEl) -> Self {
        RawElement::El(raw_html_el)
    }
}

impl From<RawSvgEl> for RawElement {
    fn from(raw_svg_el: RawSvgEl) -> Self {
        RawElement::SvgEl(raw_svg_el)
    }
}

impl<T: Into<Node>> IntoDom for RawEl<T> {
    fn into_dom(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

impl Element for RawHtmlEl {
    fn into_raw_element(self) -> RawElement {
        RawElement::El(self)
    }
}

impl Element for RawSvgEl {
    fn into_raw_element(self) -> RawElement {
        RawElement::SvgEl(self)
    }
}

// ------ ------
//  Attributes
// ------ ------

impl<'a, T> RawEl<T>
where
    T: AsRef<Node>,
    T: AsRef<EventTarget>,
    T: AsRef<web_sys::Element>,
    T: AsRef<JsValue>,
{
    pub fn attr(mut self, name: &str, value: &str) -> Self {
        self.dom_builder = self.dom_builder.attribute(name, value);
        self
    }

    pub fn attr_signal(
        mut self,
        name: impl IntoCowStr<'static>,
        value: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
    ) -> Self {
        self.dom_builder = self.dom_builder.attribute_signal(
            name.into_cow_str_wrapper(),
            value.map(|value| value.into_option_cow_str_wrapper()),
        );
        self
    }

    pub fn prop(mut self, name: &str, value: &str) -> Self {
        self.dom_builder = self.dom_builder.property(name, JsValue::from_str(value));
        self
    }

    pub fn prop_signal(
        mut self,
        name: impl IntoCowStr<'static>,
        value: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
    ) -> Self {
        self.dom_builder = self.dom_builder.property_signal(
            name.into_cow_str_wrapper(),
            value.map(|value| value.into_option_cow_str_wrapper()),
        );
        self
    }

    pub fn event_handler<E: StaticEvent>(
        mut self,
        handler: impl FnOnce(E) + Clone + 'static,
    ) -> Self {
        let handler = move |event: E| handler.clone()(event);
        self.dom_builder = self.dom_builder.event(handler);
        self
    }

    pub fn child(mut self, child: impl IntoOptionElement<'a> + 'a) -> Self {
        if let Some(child) = child.into_option_element() {
            self.dom_builder = self.dom_builder.child(child.into_raw_element().into_dom())
        }
        self
    }

    pub fn child_signal(
        mut self,
        child: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Self {
        self.dom_builder = self.dom_builder.child_signal(child.map(|child| {
            child
                .into_option_element()
                .map(|element| element.into_raw_element().into_dom())
        }));
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl IntoElement<'a> + 'a>,
    ) -> Self {
        self.dom_builder = self.dom_builder.children(
            children
                .into_iter()
                .map(|child| child.into_element().into_raw_element().into_dom()),
        );
        self
    }

    pub fn children_signal_vec(
        mut self,
        children: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Self {
        self.dom_builder = self.dom_builder.children_signal_vec(
            children.map(|child| child.into_element().into_raw_element().into_dom()),
        );
        self
    }
}
