use wasm_bindgen::JsCast;
use crate::{RenderContext, raw_el, log, Component, ApplyToComponent, render};

// ------ ------
//   Component 
// ------ ------

// component_macro!(el, El);

#[macro_export]
macro_rules! el {
    ( $($attribute:expr),* $(,)?) => {
        {
            let mut el = $crate::component::el::El::default();
            $( el = el.with($attribute); )*
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

        let state_node = raw_el(rcx, |rcx| {
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

// ------ Component ------

impl<'a, T: Component + 'a> ApplyToComponent<El<'a>> for T {
    fn apply_to_component(self, component: &mut El<'a>) {
        component.child = Some(Box::new(self));
    }
}
