use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, __TrackedCall, __TrackedCallStack, Element, IntoElement, ApplyToElement, render, element_macro};

// ------ ------
//   Element 
// ------ ------

element_macro!(col, Column::default());

#[derive(Default)]
pub struct Column<'a> {
    items: Vec<Box<dyn Element + 'a>>,
}

impl<'a> Element for Column<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        // log!("column, index: {}", rcx.index);

        let node = dom_element(rcx, |mut rcx| {
            for item in &mut self.items {
                item.render(rcx.inc_index().clone());
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

impl<'a> Column<'a> {
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

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<Column<'a>> for T {
    fn apply_to_element(self, column: &mut Column<'a>) {
        column.items.push(Box::new(self.into_element()));
    }
}
