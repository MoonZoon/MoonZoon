use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, __TrackedCall, __TrackedCallStack, Element, IntoElement, ApplyToElement, render, element_macro};
use dominator::{Dom, html};

// ------ ------
//    Element 
// ------ ------

element_macro!(row, Row::default());

#[derive(Default)]
pub struct Row {
    items: Vec<Dom>,
}

impl<'a> Element for Row {
    #[topo::nested]
    fn render(self) -> Dom {
        html!("div", {
            .class("row")
            .children(self.items)
        })
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a> Row {
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

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<Row> for T {
    fn apply_to_element(self, row: &mut Row) {
        row.items.push(self.into_element().render());
    }
}
