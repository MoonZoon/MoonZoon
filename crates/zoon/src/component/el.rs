use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, log, Component, IntoComponent, ApplyToComponent, render};

// ------ ------
//   Component 
// ------ ------

// component_macro!(el, El);

#[macro_export]
macro_rules! el {
    ( $($attribute:expr),* $(,)?) => {
        {
            #[allow(unused_mut)]
            let mut el = $crate::component::el::El::default();
            $( $attribute.apply_to_component(&mut el); )*
            el
        }
    }
}

#[derive(Default)]
pub struct El<'a> {
    child: Option<Box<dyn Component + 'a>>,
}

impl<'a> Component for El<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        log!("el, index: {}", rcx.index);

        let state_node = dom_element(rcx, |rcx| {
            if let Some(child) = self.child.as_mut() {
                child.render(rcx)
            }
        });
        state_node.update_mut(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "el").unwrap();
        });
    }
}

// ------ ------
//  Attributes 
// ------ ------

// ------ IntoComponent ------

impl<'a, T: IntoComponent<'a> + 'a> ApplyToComponent<El<'a>> for T {
    fn apply_to_component(self, element: &mut El<'a>) {
        element.child = Some(Box::new(self.into_component()));
    }
}
