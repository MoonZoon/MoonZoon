use wasm_bindgen::JsCast;
use crate::{Cx, raw_el, log};
use crate::controls::Control;

#[macro_export]
macro_rules! column {
    ( $($item:expr),* $(,)?) => {
        {
            let mut column = column::Column::new();
            $(
                $item.apply_to_column(&mut column);
            )*
            column
        }
    }
}

#[derive(Default)]
pub struct Column<'a> {
    children: Vec<Box<dyn Control + 'a>>,
}

impl<'a> Column<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> Control for Column<'a> {
    #[topo::nested]
    fn build(&mut self, cx: Cx) {
        log!("column, index: {}", cx.index);

        let state_node = raw_el(cx, |mut cx| {
            for child in &mut self.children {
                child.build(cx.inc_index().clone());
            }
        });
        state_node.update(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "column");
        });
    }
}

pub trait ApplyToColumn<'a> {
    fn apply_to_column(self, column: &mut Column<'a>);
}

impl<'a, T: ApplyToColumn<'a>> ApplyToColumn<'a> for Option<T> {
    fn apply_to_column(self, column: &mut Column<'a>) {
        if let Some(applicable_to_column) = self {
            applicable_to_column.apply_to_column(column)
        }
    }
} 

impl<'a, T: Control + 'a> ApplyToColumn<'a> for T {
    fn apply_to_column(self, column: &mut Column<'a>) {
        column.children.push(Box::new(self));
    }
} 
