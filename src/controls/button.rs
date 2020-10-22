use wasm_bindgen::JsCast;
use crate::{Cx, raw_el, log};
use crate::controls::Control;

#[macro_export]
macro_rules! button {
    ( $($item:expr),* $(,)?) => {
        {
            let mut button = button::Button::new();
            $(
                $item.apply_to_button(&mut button);
            )*
            button
        }
    }
}

#[derive(Default)]
pub struct Button<'a> {
    label: Option<Box<dyn Control + 'a>>,
    on_press: Option<OnPress<'a>>,
}

impl<'a> Button<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> Control for Button<'a> {
    #[topo::nested]
    fn build(&mut self, cx: Cx) {
        log!("button, index: {}", cx.index);

        let state_node = raw_el(cx, |cx: Cx| {
            if let Some(label) = self.label.as_mut() {
                label.build(cx);
            }
        });
        state_node.update(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "button");
            element.set_attribute("role", "button");
            element.set_attribute("tabindex", "0");
        });
    }
}

pub trait ApplyToButton<'a> {
    fn apply_to_button(self, button: &mut Button<'a>);
}

impl<'a, T: Control + 'a> ApplyToButton<'a> for T {
    fn apply_to_button(self, button: &mut Button<'a>) {
         button.label = Some(Box::new(self));
    }
} 

pub struct OnPress<'a>(Box<dyn FnOnce() + 'a>);
pub fn on_press<'a>(on_press: impl FnOnce() + 'a) -> OnPress<'a> {
    OnPress(Box::new(on_press))
}
impl<'a> ApplyToButton<'a> for OnPress<'a> {
    fn apply_to_button(self, button: &mut Button<'a>) {
        button.on_press = Some(self);
    }
}
