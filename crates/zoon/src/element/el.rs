use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, Element, IntoElement, ApplyToElement, render, element_macro};

// ------ ------
//   Element 
// ------ ------

element_macro!(el, El::default());

#[derive(Default)]
pub struct El<'a> {
    child: Option<Box<dyn Element + 'a>>,
}

impl<'a> Element for El<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        // log!("el, index: {}", rcx.index);

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

// ------ IntoElement ------

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<El<'a>> for T {
    fn apply_to_element(self, element: &mut El<'a>) {
        element.child = Some(Box::new(self.into_element()));
    }
}
