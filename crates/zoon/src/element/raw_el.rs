use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, Element, __TrackedCall, __TrackedCallStack, IntoElement, ApplyToElement, render, element_macro};

// ------ ------
//   Element 
// ------ ------

element_macro!(raw_el, RawEl::default());

#[derive(Default)]
pub struct RawEl<'a> {
    tag: Option<&'a str>,
    attrs: Vec<Attr<'a>>,
    event_handlers: Vec<EventHandler<'a>>,
    child: Option<Box<dyn Element + 'a>>,
}

impl<'a> Element for RawEl<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        // log!("raw_el, index: {}", rcx.index);

        let node = dom_element(rcx, |rcx| {
            if let Some(child) = self.child.as_mut() {
                child.render(rcx)
            }
        });
        node.update_mut(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "el").unwrap();
        });
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a> RawEl<'a> {
    pub fn child(mut self, child: impl IntoElement<'a> + 'a) -> Self {
        child.into_element().apply_to_element(&mut self);
        self
    }
} 

// ------ IntoElement ------

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<RawEl<'a>> for T {
    fn apply_to_element(self, element: &mut RawEl<'a>) {
        element.child = Some(Box::new(self.into_element()));
    }
}

// ------ raw_el::tag(...)

pub struct Tag<'a>(&'a str);
pub fn tag<'a>(tag: &'a str) -> Tag<'a> {
    Tag(tag)
}
impl<'a> ApplyToElement<RawEl<'a>> for Tag<'a> {
    fn apply_to_element(self, raw_el: &mut RawEl<'a>) {
        raw_el.tag = Some(self.0);
    }
}

// ------ raw_el::attr(...)

pub struct Attr<'a> {
    name: &'a str,
    value: &'a str
}
pub fn attr<'a>(name: &'a str, value: &'a str) -> Attr<'a> {
    Attr { name, value }
}
impl<'a> ApplyToElement<RawEl<'a>> for Attr<'a> {
    fn apply_to_element(self, raw_el: &mut RawEl<'a>) {
        raw_el.attrs.push(self);
    }
}

// ------ raw_el::event_handler(...)

pub struct EventHandler<'a> {
    event: &'a str,
    handler: Box<dyn Fn(web_sys::Event)>
}
pub fn event_handler(event: &str, handler: impl FnOnce(web_sys::Event) + Clone + 'static) -> EventHandler {
    EventHandler {
        event,
        handler: Box::new(move |event| handler.clone()(event))
    }
}
impl<'a> ApplyToElement<RawEl<'a>> for EventHandler<'a> {
    fn apply_to_element(self, raw_el: &mut RawEl<'a>) {
        raw_el.event_handlers.push(self);
    }
}
