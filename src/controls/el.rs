use wasm_bindgen::JsCast;
use crate::{Cx, raw_el, log};

#[macro_export]
macro_rules! el {
    ( $($item:expr),* $(,)?) => {
        {
            let mut el = el::El::new();
            $(
                $item.apply_to_el(&mut el);
            )*
            el
        }
    }
}

#[derive(Default)]
pub struct El {
    child: Option<Child>,
}

impl El {
    pub fn new() -> Self {
        Self::default()
    }

    #[topo::nested]
    pub fn build(self, cx: Cx) {
        log!("el, index: {}", cx.index);

        let state_node = raw_el(cx, |cx| {
            if let Some(child) = self.child {
                (child.0)(cx)
            }
        });
        state_node.update(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "el");
        });
    }
}

pub trait ApplyToEl {
    fn apply_to_el(self, el: &mut El);
}

pub struct Child(Box<dyn FnOnce(Cx)>);
pub fn child(child: impl FnOnce(Cx) + 'static) -> Child {
    Child(Box::new(child))
}
impl ApplyToEl for Child {
    fn apply_to_el(self, el: &mut El) {
        el.child = Some(self);
    }
}
