use wasm_bindgen::JsCast;
use crate::{Cx, raw_el, log};
use crate::controls::Control;

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
}

impl<'a> Control for Row<'a> {
    #[topo::nested]
    fn build(&mut self, cx: Cx) {
        log!("row, index: {}", cx.index);

        let state_node = raw_el(cx, |mut cx| {
            if let Some(children) = self.children.take() {
                for mut child in children.0 {
                    child.build(cx.inc_index().clone());
                }
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

impl<'a, T: Control + 'a> ApplyToRow<'a> for T {
    fn apply_to_row(self, row: &mut Row<'a>) {
        if let Some(children) = &mut row.children {
            children.0.push(Box::new(self));
        } else {
            row.children = Some(Children(vec![Box::new(self)]));
        }
    }
}

impl<'a, T: Control + 'a> ApplyToRow<'a> for Option<T> {
    fn apply_to_row(self, row: &mut Row<'a>) {
        if let Some(control) = self {
            control.apply_to_row(row);
        }
    }
} 

pub struct Children<'a>(Vec<Box<dyn Control + 'a>>);
pub fn children<'a>(children: Vec<Box<dyn Control + 'a>>) -> Children<'a> {
    Children(children)
}
impl<'a> ApplyToRow<'a> for Children<'a> {
    fn apply_to_row(self, row: &mut Row<'a>) {
        row.children = Some(self);
    }
}
