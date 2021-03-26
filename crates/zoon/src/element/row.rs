use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, __TrackedCall, __TrackedCallStack, Element, IntoElement, ApplyToElement, render, element_macro};

// ------ ------
//    Element 
// ------ ------

element_macro!(row, Row::default());

#[derive(Default)]
pub struct Row<'a> {
    items: Vec<Box<dyn Element + 'a>>,
}

impl<'a> Element for Row<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        // log!("row, index: {}", rcx.index);

        let node = dom_element(rcx, |mut rcx| {
            for item in &mut self.items {
                item.render(rcx.inc_index().clone());
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

impl<'a> Row<'a> {
    pub fn item(mut self, item: impl IntoElement<'a> + 'a) -> Self {
        item.into_element().apply_to_element(&mut self);
        self
    }

    pub fn items<IE: IntoElement<'a> + 'a>(mut self, items: impl IntoIterator<Item = IE>) -> Self {
        for item in items.into_iter() {
            item.into_element().apply_to_element(&mut self);
        }
        self
    }
} 

// ------ IntoElement ------

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<Row<'a>> for T {
    fn apply_to_element(self, row: &mut Row<'a>) {
        row.items.push(Box::new(self.into_element()));
    }
}
