use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, log, Component, IntoComponent, ApplyToComponent, render, component_macro};

// ------ ------
//   Component 
// ------ ------

component_macro!(row, Row::default());

#[derive(Default)]
pub struct Row<'a> {
    children: Vec<Box<dyn Component + 'a>>,
}

impl<'a> Component for Row<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        log!("row, index: {}", rcx.index);

        let state_node = dom_element(rcx, |mut rcx| {
            for child in &mut self.children {
                child.render(rcx.inc_index().clone());
            }
        });
        state_node.update_mut(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "row").unwrap();
        });
    }
}

// ------ ------
//  Attributes 
// ------ ------

// ------ IntoComponent ------

impl<'a, T: IntoComponent<'a> + 'a> ApplyToComponent<Row<'a>> for T {
    fn apply_to_component(self, row: &mut Row<'a>) {
        row.children.push(Box::new(self.into_component()));
    }
}
