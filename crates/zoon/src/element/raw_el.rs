use crate::*;

// ------ ------
//   Element 
// ------ ------

pub struct RawEl {
    dom_builder: DomBuilder<web_sys::HtmlElement>,
}

impl RawEl {
    pub fn new(tag: &str) -> Self {
        Self {
            dom_builder: DomBuilder::new_html(tag)
        }
    }
}

impl From<RawEl> for RawElement {
    fn from(raw_el: RawEl) -> Self {
        RawElement::El(raw_el)
    }
}

impl IntoDom for RawEl {
    fn into_dom(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

impl Element for RawEl {
    fn into_raw_element(self) -> RawElement {
        self.into()
    }
}
    
// ------ ------
//  Attributes 
// ------ ------

impl<'a> RawEl {
    pub fn attr(mut self, name: &str, value: &str) -> Self {
        self.dom_builder = self.dom_builder.attribute(name, value);
        self
    }

    pub fn attr_signal(mut self, name: impl ToString, value: impl Signal<Item = Option<impl ToString>> + Unpin + 'static) -> Self {
        self.dom_builder = self.dom_builder.attribute_signal(
            name.to_string(), 
            value.map(|value| value.map(|value| value.to_string()))
        );
        self
    }

    pub fn event_handler<E: StaticEvent>(mut self, handler: impl FnOnce(E) + Clone + 'static) -> Self {
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
        child: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static
    ) -> Self {
        self.dom_builder = self.dom_builder.child_signal(
            child.map(|child| child.into_option_element().map(|element| {
                element.into_raw_element().into_dom()
            }))
        );
        self
    }

    pub fn children(
        mut self, 
        children: impl IntoIterator<Item = impl IntoElement<'a> + 'a>
    ) -> Self {
        self.dom_builder = self.dom_builder.children(
            children.into_iter().map(|child| child.into_element().into_raw_element().into_dom())
        );
        self
    }

    pub fn children_signal_vec(
        mut self, 
        children: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static
    ) -> Self {
        self.dom_builder = self.dom_builder.children_signal_vec(
            children.map(|child| child.into_element().into_raw_element().into_dom())
        );
        self
    }
}
