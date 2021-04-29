use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, __TrackedCall, __TrackedCallStack, Element, IntoElement, ApplyToElement, render, element_macro};
use dominator::{Dom, class, html, clone, events, text, DomBuilder};
use futures_signals::signal_vec::{SignalVec, SignalVecExt};

// ------ ------
//   Element 
// ------ ------

element_macro!(col, Column::default());

#[derive(Default)]
pub struct Column {
    items: Vec<Dom>,
    items_signal: Option<Box<dyn SignalVec<Item = Dom> + Unpin>>
}

impl Element for Column {
    #[topo::nested]
    fn render(self) -> Dom {
        let mut builder = DomBuilder::<web_sys::HtmlElement>::new_html("div")
            .class("column");

        if !self.items.is_empty() {
            builder = builder
                .children(self.items);
        }

        if let Some(items_signal) = self.items_signal {
            builder = builder
                .children_signal_vec(items_signal);
        }

        builder.into_dom()
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

    pub fn items_signal<IE: IntoElement<'a> + 'a>(mut self, items: impl SignalVec<Item = IE> + Unpin + 'static) -> Self {
        self.items_signal = Some(Box::new(items.map(|item| item.into_element().render())));
        self
    }
} 

// ------ IntoElement ------

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<Column> for T {
    fn apply_to_element(self, column: &mut Column) {
        column.items.push(self.into_element().render());
    }
}
