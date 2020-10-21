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
pub struct Row<'a> {
    children: Option<Children<'a>>,
}

impl<'a> Row<'a> {
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

pub trait ApplyToRow<'a> {
    fn apply_to_row(self, row: &mut Row<'a>);
}

pub struct Children<'a>(Box<dyn FnOnce(Cx) + 'a>);
pub fn children<'a>(children: impl FnOnce(Cx) + 'a) -> Children<'a> {
    Children(Box::new(children))
}
impl<'a> ApplyToRow<'a> for Children<'a> {
    fn apply_to_row(self, row: &mut Row<'a>) {
        row.children = Some(self);
    }
}
