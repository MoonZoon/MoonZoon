use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, __TrackedCall, __TrackedCallStack, Element, IntoElement, ApplyToElement, render, element_macro};
use dominator::{Dom, class, html, clone, events, text};

// ------ ------
//   Element 
// ------ ------

element_macro!(col, Column::default());

#[derive(Default)]
pub struct Column {
    items: Vec<Dom>,
}

impl Element for Column {
    #[topo::nested]
    fn render(self) -> Dom {
        html!("div", {
            .class("column")
            .children(self.items)
        })
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a> Column {
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

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<Column> for T {
    fn apply_to_element(self, column: &mut Column) {
        column.items.push(self.into_element().render());
    }
}
