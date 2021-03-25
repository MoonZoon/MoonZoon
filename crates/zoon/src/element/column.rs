use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, __TrackedCall, __TrackedCallStack, Element, IntoElement, ApplyToElement, render, element_macro};

// ------ ------
//   Element 
// ------ ------

element_macro!(col, Column::default());

#[derive(Default)]
pub struct Column<'a> {
    children: Vec<Box<dyn Element + 'a>>,
}

impl<'a> Element for Column<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        // log!("column, index: {}", rcx.index);

        let node = dom_element(rcx, |mut rcx| {
            for child in &mut self.children {
                child.render(rcx.inc_index().clone());
            }
        });
        node.update_mut(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "column").unwrap();
        });
    }
}

// ------ ------
//  Attributes 
// ------ ------

// ------ IntoElement ------

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<Column<'a>> for T {
    fn apply_to_element(self, column: &mut Column<'a>) {
        column.children.push(Box::new(self.into_element()));
    }
}
