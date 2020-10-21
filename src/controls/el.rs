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
pub struct El<'a> {
    child: Option<Child<'a>>,
}

impl<'a> El<'a> {
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

pub trait ApplyToEl<'a> {
    fn apply_to_el(self, el: &mut El<'a>);
}

pub struct Child<'a>(Box<dyn FnOnce(Cx) + 'a>);
pub fn child<'a>(child: impl FnOnce(Cx) + 'a) -> Child<'a> {
    Child(Box::new(child))
}
impl<'a> ApplyToEl<'a> for Child<'a> {
    fn apply_to_el(self, el: &mut El<'a>) {
        el.child = Some(self);
    }
}
