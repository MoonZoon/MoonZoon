use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, __TrackedCall, __TrackedCallStack, Element, IntoElement, ApplyToElement, render, element_macro};
use dominator::{Dom, class, html, clone, events, text, DomBuilder};
use futures_signals::{signal::{Signal, SignalExt}, signal_vec::{SignalVec, SignalVecExt}};

// ------ ------
//   Element 
// ------ ------

element_macro!(col, Column::default());

#[derive(Default)]
pub struct Column {
    items: Vec<Item>,
    items_signal_vec: Option<Box<dyn SignalVec<Item = Dom> + Unpin>>
}

enum Item {
    Static(Dom),
    Dynamic(Box<dyn Signal<Item = Option<Dom>> + Unpin>),
}

impl Element for Column {
    fn render(self) -> Dom {
        let mut builder = DomBuilder::<web_sys::HtmlElement>::new_html("div")
            .class("column");

        for item in self.items {
            builder = match item {
                Item::Static(child) => builder.child(child),
                Item::Dynamic(child) => builder.child_signal(child),
            }
        }

        if let Some(items_signal_vec) = self.items_signal_vec {
            builder = builder
                .children_signal_vec(items_signal_vec);
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

    pub fn item_signal<IE: IntoElement<'a> + 'a>(mut self, item: impl Signal<Item = IE> + Unpin + 'static) -> Self {
        self::item_signal(item).apply_to_element(&mut self);
        self
    }

    pub fn items_signal_vec<IE: IntoElement<'a> + 'a>(mut self, items: impl SignalVec<Item = IE> + Unpin + 'static) -> Self {
        self::items_signal_vec(items).apply_to_element(&mut self);
        self
    }
} 

// ------ IntoElement ------

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<Column> for T {
    fn apply_to_element(self, column: &mut Column) {
        column.items.push(Item::Static(self.into_element().render()));
    }
}

// ------ column::signal(...) ------

pub struct ItemSignal(Box<dyn Signal<Item = Option<Dom>> + Unpin>);
pub fn item_signal<'a, IE: IntoElement<'a> + 'a>(item: impl Signal<Item = IE> + Unpin + 'static) -> ItemSignal {
    ItemSignal(Box::new(item.map(|item| Some(item.into_element().render()))))
}
impl ApplyToElement<Column> for ItemSignal {
    fn apply_to_element(self, column: &mut Column) {
        column.items.push(Item::Dynamic(self.0));
    }
}

// ------ column::items_signal_vec(...) ------
pub struct ItemsSignalVec(Box<dyn SignalVec<Item = Dom> + Unpin>);
pub fn items_signal_vec<'a, IE: IntoElement<'a> + 'a>(items: impl SignalVec<Item = IE> + Unpin + 'static) -> ItemsSignalVec {
    ItemsSignalVec(Box::new(items.map(|item| item.into_element().render())))
}
impl ApplyToElement<Column> for ItemsSignalVec {
    fn apply_to_element(self, column: &mut Column) {
        column.items_signal_vec = Some(self.0);
    }
}
