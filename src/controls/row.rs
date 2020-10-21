use wasm_bindgen::JsCast;
use crate::{Cx, raw_el, log};

#[macro_export]
macro_rules! row {
    ( $($item:expr),* $(,)?) => {
        {
            let mut row = row::Row::new();
            $(
                $item.apply_to_row(&mut row);
            )*
            row
        }
    }
}

#[derive(Default)]
pub struct Row {
    children: Option<Children>,
}

impl Row {
    pub fn new() -> Self {
        Self::default()
    }

    #[topo::nested]
    pub fn build(self, cx: Cx) {
        log!("row, index: {}", cx.index);

        let state_node = raw_el(cx, |cx| {
            if let Some(children) = self.children {
                (children.0)(cx)
            }
        });
        state_node.update(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "row");
        });
    }
}

pub trait ApplyToRow {
    fn apply_to_row(self, row: &mut Row);
}

pub struct Children(Box<dyn FnOnce(Cx)>);
pub fn children(children: impl FnOnce(Cx) + 'static) -> Children {
    Children(Box::new(children))
}
impl ApplyToRow for Children {
    fn apply_to_row(self, row: &mut Row) {
        row.children = Some(self);
    }
}
