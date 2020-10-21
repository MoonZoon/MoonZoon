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
    children: Option<Children<'a>>,
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
            if let Some(children) = self.children.take() {
                for mut child in children.0 {
                    child.build(cx.inc_index().clone());
                }
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

pub struct Children<'a>(Vec<Box<dyn Control + 'a>>);
pub fn children<'a>(children: Vec<Box<dyn Control + 'a>>) -> Children<'a> {
    Children(children)
}
impl<'a> ApplyToColumn<'a> for Children<'a> {
    fn apply_to_column(self, column: &mut Column<'a>) {
        column.children = Some(self);
    }
}
