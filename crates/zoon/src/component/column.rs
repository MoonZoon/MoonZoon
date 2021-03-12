use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, log, Component, IntoComponent, ApplyToComponent, render};

// ------ ------
//   Component 
// ------ ------

// component_macro!(col, Column);

#[macro_export]
macro_rules! col {
    ( $($attribute:expr),* $(,)?) => {
        {
            let mut column = $crate::component::column::Column::default();
            $( column = column.with($attribute); )*
            column
        }
    }
}

#[derive(Default)]
pub struct Column<'a> {
    children: Vec<Box<dyn Component + 'a>>,
}

impl<'a> Component for Column<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        log!("column, index: {}", rcx.index);

        let state_node = dom_element(rcx, |mut rcx| {
            for child in &mut self.children {
                child.render(rcx.inc_index().clone());
            }
        });
        state_node.update_mut(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "column").unwrap();
        });
    }
}

// ------ ------
//  Attributes 
// ------ ------

// ------ IntoComponent ------

impl<'a, T: IntoComponent<'a> + 'a> ApplyToComponent<Column<'a>> for T {
    fn apply_to_component(self, column: &mut Column<'a>) {
        column.children.push(Box::new(self.into_component()));
    }
}
