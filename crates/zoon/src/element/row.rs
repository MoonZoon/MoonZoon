use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, __TrackedCall, __TrackedCallStack, Element, IntoElement, ApplyToElement, render, element_macro};
use crate::log;
use tracked_call_macro::tracked_call;

// ------ ------
//    Element 
// ------ ------

element_macro!(row, Row::default());

#[derive(Default)]
pub struct Row<'a> {
    children: Vec<Box<dyn Element + 'a>>,
}

impl<'a> Element for Row<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        // log!("row, index: {}", rcx.index);

        let node = dom_element(rcx, |mut rcx| {
            for child in &mut self.children {
                child.render(rcx.inc_index().clone());
            }
        });
        node.update_mut(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "row").unwrap();
        });
    }
}

// ------ ------
//  Attributes 
// ------ ------

// ------ IntoElement ------

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<Row<'a>> for T {
    fn apply_to_element(self, row: &mut Row<'a>) {
        row.children.push(Box::new(self.into_element()));
    }
}
