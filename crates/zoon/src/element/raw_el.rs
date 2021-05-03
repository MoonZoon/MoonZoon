use crate::*;

// ------ ------
//   Element 
// ------ ------

pub struct RawEl {
    dom_builder: DomBuilder<web_sys::HtmlElement>,
}

impl RawEl {
    pub fn with_tag(tag: &str) -> Self {
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
    pub fn attr(self, name: &str, value: &str) -> Self {
        Self {
            dom_builder: self.dom_builder.attribute(name, value)
        }
    }

    pub fn attr_signal(self, name: impl ToString, value: impl Signal<Item = Option<impl ToString>> + Unpin + 'static) -> Self {
        Self {
            dom_builder: self.dom_builder.attribute_signal(
                name.to_string(), 
                value.map(|value| value.map(|value| value.to_string()))
            )
        }
    }

    pub fn event_handler<E: StaticEvent>(self, handler: impl FnOnce(E) + Clone + 'static) -> Self {
        let handler = move |event: E| handler.clone()(event);
        Self {
            dom_builder: self.dom_builder.event(handler)
        }
    }

    pub fn child(self, child: impl IntoOptionElement<'a> + 'a) -> Self {
        let dom_builder = if let Some(child) = child.into_option_element() {
            self.dom_builder.child(child.into_raw_element().into_dom())
        } else {
            self.dom_builder
        };
        Self {
            dom_builder,
        }
    }

    pub fn child_signal(
        self, 
        child: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static
    ) -> Self {
        Self {
            dom_builder: self.dom_builder.child_signal(
                child.map(|child| child.into_option_element().map(|element| {
                    element.into_raw_element().into_dom()
                }))
            ),
        }
    }

    pub fn children(self, 
        children: impl IntoIterator<Item = impl IntoElement<'a> + 'a>
    ) -> Self {
        Self {
            dom_builder: self.dom_builder.children(
                children.into_iter().map(|child| child.into_element().into_raw_element().into_dom())
            ),
        }
    }

    pub fn children_signal_vec(
        self, 
        children: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static
    ) -> Self {
        Self {
            dom_builder: self.dom_builder.children_signal_vec(
                children.map(|child| child.into_element().into_raw_element().into_dom())
            ),
        }
    }
}
