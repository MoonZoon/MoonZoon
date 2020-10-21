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

        let state_node = raw_el(cx, |cx| {
            if let Some(children) = self.children.take() {
                (children.0)(cx)
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

pub struct Children<'a>(Box<dyn FnOnce(Cx) + 'a>);
pub fn children<'a>(children: impl FnOnce(Cx) + 'a) -> Children<'a> {
    Children(Box::new(children))
}
impl<'a> ApplyToColumn<'a> for Children<'a> {
    fn apply_to_column(self, column: &mut Column<'a>) {
        column.children = Some(self);
    }
}
