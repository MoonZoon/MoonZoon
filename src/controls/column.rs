use wasm_bindgen::JsCast;
use crate::{Cx, raw_el, log};

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
pub struct Column {
    children: Option<Children>,
}

impl Column {
    pub fn new() -> Self {
        Self::default()
    }

    #[topo::nested]
    pub fn build(self, cx: Cx) {
        log!("column, index: {}", cx.index);

        let state_node = raw_el(cx, |cx| {
            if let Some(children) = self.children {
                (children.0)(cx)
            }
        });
        state_node.update(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "column");
        });
    }
}

pub trait ApplyToColumn {
    fn apply_to_column(self, column: &mut Column);
}

pub struct Children(Box<dyn FnOnce(Cx)>);
pub fn children(children: impl FnOnce(Cx) + 'static) -> Children {
    Children(Box::new(children))
}
impl ApplyToColumn for Children {
    fn apply_to_column(self, column: &mut Column) {
        column.children = Some(self);
    }
}
