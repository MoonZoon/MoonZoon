use wasm_bindgen::JsCast;
use crate::{RenderContext, raw_el, log, Component, ApplyToComponent, render};

// ------ ------
//   Component 
// ------ ------

// component_macro!(row, Row);

#[macro_export]
macro_rules! row {
    ( $($attribute:expr),* $(,)?) => {
        {
            let mut row = $crate::component::row::Row::default();
            $( row = row.with($attribute); )*
            row
        }
    }
}

#[derive(Default)]
pub struct Row<'a> {
    children: Vec<Box<dyn Component + 'a>>,
}

impl<'a> Component for Row<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        log!("row, index: {}", rcx.index);

        let state_node = raw_el(rcx, |mut rcx| {
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

// ------ Component ------

impl<'a, T: Component + 'a> ApplyToComponent<Row<'a>> for T {
    fn apply_to_component(self, component: &mut Row<'a>) {
        component.children.push(Box::new(self));
    }
}
