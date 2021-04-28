use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, Element, __TrackedCall, __TrackedCallStack, IntoElement, ApplyToElement, render, element_macro};
use dominator::{Dom, html, DomBuilder};
use futures_signals::signal::{Signal, SignalExt};

// ------ ------
//   Element 
// ------ ------

element_macro!(el, El::default());

#[derive(Default)]
pub struct El {
    child: Option<Dom>,
    child_signal: Option<Box<dyn Signal<Item = Option<Dom>> + Unpin>>,
}

impl Element for El {
    #[topo::nested]
    fn render(self) -> Dom {
        let mut builder = DomBuilder::<web_sys::HtmlElement>::new_html("div")
            .class("el");

        if let Some(child) = self.child {
            builder = builder
                .child(child);
        }

        if let Some(child_signal) = self.child_signal {
            builder = builder
                .child_signal(child_signal);
        }

        builder.into_dom()
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a> El {
    pub fn child(mut self, child: impl IntoElement<'a> + 'a) -> Self {
        child.into_element().apply_to_element(&mut self);
        self
    }

    pub fn child_signal(mut self, child: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static) -> Self {
        self.child_signal = Some(
            Box::new(
                child.map(|child| Some(child.into_element().render()))
            )
        );
        self
    }
} 

// ------ IntoElement ------

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<El> for T {
    fn apply_to_element(self, element: &mut El) {
        element.child = Some(self.into_element().render());
    }
}
